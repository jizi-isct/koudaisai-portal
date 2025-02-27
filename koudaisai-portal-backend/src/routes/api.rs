mod forms;

use crate::routes::AppState;
use axum::Router;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "init /api")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new().nest("/v1/forms", forms::init_router())
}
