mod api;
use crate::config::{Secrets, Web};
use crate::middlewares;
use crate::util::jwt::JWTManager;
use crate::util::oidc::OIDCClient;
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::get_service;
use jsonwebtoken::Algorithm;
use reqwest::Client;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing::{debug, instrument};

/// 静的配信(SPA) と `/api/plans_info`(外部 API プロキシ、未移行)のみを提供する。
/// 旧 `/api/v2/*` は `api_v3` へ、旧 `/auth/*` は `auth_v2` へ移行済みのため撤去した。
/// `/api/plans_info` は移行先が無いため legacy 認証ミドルウェア(`middlewares::auth`)配下で維持する。
#[instrument(skip(web, oidc_client, secrets))]
pub fn init_routes(web: &Web, oidc_client: OIDCClient, secrets: Secrets) -> Router {
    debug!("Initializing legacy routes (static + plans_info)");
    let state = Arc::new(AppState {
        oidc_client,
        http_client: Client::new(),
        jwt_manager: JWTManager::new(
            Algorithm::RS256,
            600,
            60 * 60 * 24 * 30 * 6,
            60 * 10, // 10 minutes
            "https://portal.koudaisai.jp",
            web.auth.get_jwt_encoding_key().unwrap(),
            web.auth.get_jwt_decoding_key().unwrap(),
        ),
    });

    let serve_dir =
        ServeDir::new(&web.static_files.web_path).append_index_html_on_directories(true);

    Router::new()
        // `/api` のワイルドカード nest は使わない(/api/v3 と衝突するため)。
        // plans_info のみ個別パスでマウントする。
        .nest_service("/api/plans_info", api::plans_info::init_service(secrets))
        .fallback_service(get_service(serve_dir))
        .route_layer(from_fn_with_state(state.clone(), middlewares::auth))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

pub struct AppState {
    pub oidc_client: OIDCClient,
    pub http_client: Client,
    pub jwt_manager: JWTManager,
}
