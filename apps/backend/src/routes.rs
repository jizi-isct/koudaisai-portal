mod api;
mod auth;
use crate::config::{Sendgrid, Web};
use crate::middlewares;
use crate::service::discord::Discord;
use crate::util::jwt::JWTManager;
use crate::util::oidc::OIDCClient;
use crate::util::sha::SHAManager;
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::middleware::from_fn_with_state;
use axum::routing::get_service;
use axum::Router;
use jsonwebtoken::Algorithm;
use oauth2::PkceCodeVerifier;
use openidconnect::Nonce;
use reqwest::Client;
use sea_orm::DatabaseConnection;
use sendgrid::v3::{Email, Sender};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing::{debug, instrument};

#[instrument(skip(web))]
pub fn init_routes(
    web: &Web,
    sendgrid: Sendgrid,
    db_conn: DatabaseConnection,
    oidc_client: OIDCClient,
    s3_client: aws_sdk_s3::Client,
    s3_bucket: String,
    discord: Discord,
) -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    debug!("Initializing routes");
    let state = Arc::new(AppState {
        web: web.clone(),
        sendgrid_sender_email: Email::new(sendgrid.sender_address.clone()),
        sendgrid_sender: Sender::new(sendgrid.api_key, None),
        db_conn: db_conn.clone(),
        oidc_client,
        auth_sessions: Default::default(),
        http_client: Client::new(),
        jwt_manager: JWTManager::new(
            Algorithm::RS256,
            600,
            60 * 60 * 24 * 30 * 6,
            60 * 10, // 10 minutes
            "https://portal.koudaisai.jp",
            web.auth.get_jwt_encoding_key().unwrap(),
            web.auth.get_jwt_decoding_key().unwrap(),
            db_conn,
        ),
        sha_manager: SHAManager {
            stretch_cost: 2_i32.pow(web.auth.stretch_cost as u32),
        },
        s3_client,
        s3_bucket,
        discord,
    });

    let serve_dir =
        ServeDir::new(&web.static_files.web_path).append_index_html_on_directories(true);
    // let admin_serve_dir =
    //     ServeDir::new(&web.static_files.admin_path).append_index_html_on_directories(true);

    Router::new()
        .nest("/auth", auth::init_router())
        .nest("/api", api::init_router())
        .fallback_service(get_service(serve_dir))
        // .nest_service("/admin", get_service(admin_serve_dir))
        .route_layer(from_fn_with_state(state.clone(), middlewares::auth))
        .layer(CorsLayer::permissive())
        .with_state(state)
        .into_make_service_with_connect_info::<SocketAddr>()
}

pub struct AppState {
    pub web: Web,
    pub sendgrid_sender_email: Email,
    pub sendgrid_sender: Sender,
    pub db_conn: DatabaseConnection,
    pub oidc_client: OIDCClient,
    pub auth_sessions: Mutex<HashMap<String, AuthSession>>,
    pub http_client: Client,
    pub jwt_manager: JWTManager,
    pub sha_manager: SHAManager,
    pub s3_client: aws_sdk_s3::Client,
    pub s3_bucket: String,
    pub discord: Discord,
}

pub struct AuthSession {
    pkce_verifier: PkceCodeVerifier,
    nonce: Nonce,
}
