mod approval_requests;
mod document_categories;
mod documents;
mod exhibitors;
mod files;
mod forms;
mod notifications;
mod users;
mod util;

use crate::routes::AppState;
use axum::Router;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "init /api")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/v1/forms", forms::init_router())
        .nest("/v1/exhibitors", exhibitors::init_router())
        .nest("/v1/documents", documents::init_router())
        .nest(
            "/v1/document-categories",
            document_categories::init_router(),
        )
        .nest("/v1/files", files::init_router())
        .nest("/v1/users", users::init_router())
        .nest("/v1/notifications", notifications::init_router())
        .nest("/v1/util", util::init_router())
        .nest("/v1/approval-requests", approval_requests::init_router())
}
