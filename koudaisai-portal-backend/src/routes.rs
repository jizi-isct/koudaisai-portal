mod auth;

use crate::config::Web;
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::Router;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, instrument};

#[instrument(skip(web))]
pub fn init_routes(
    web: &Web,
    db_conn: DatabaseConnection,
) -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    debug!("Initializing routes");
    let state = Arc::new(AppState {
        web: web.clone(),
        db_conn,
        jwt_encoding_key: web.auth.get_jwt_encoding_key().unwrap(),
        jwt_decoding_key: web.auth.get_jwt_decoding_key().unwrap(),
    });
    Router::new()
        .nest("/auth", auth::init_router(Arc::clone(&state)))
        .with_state(state)
        .into_make_service_with_connect_info::<SocketAddr>()
}

struct AppState {
    pub web: Web,
    pub db_conn: DatabaseConnection,
    pub jwt_encoding_key: EncodingKey,
    pub jwt_decoding_key: DecodingKey,
}
