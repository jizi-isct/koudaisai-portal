mod auth;

use crate::config::Web;
use axum::Router;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use axum::routing::IntoMakeService;
use tracing::{debug, instrument};

#[instrument(skip(web))]
pub fn init_routes(web: &Web, db_conn: DatabaseConnection) -> IntoMakeService<Router> {
    debug!("Initializing routes");
    let state = Arc::new(AppState {
        web: web.clone(),
        db_conn,
    });
    Router::new()
        .nest("/auth", auth::init_router(Arc::clone(&state)))
        .with_state(state)
        .into_make_service()
}

#[derive(Debug)]
struct AppState {
    pub web: Web,
    pub db_conn: DatabaseConnection,
}
