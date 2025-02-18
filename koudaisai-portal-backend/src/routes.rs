mod auth;

use crate::config::Web;
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::Router;
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
    });
    Router::new()
        .nest("/auth", auth::init_router(Arc::clone(&state)))
        .with_state(state)
        .into_make_service_with_connect_info::<SocketAddr>()
}

#[derive(Debug)]
struct AppState {
    pub web: Web,
    pub db_conn: DatabaseConnection,
}
