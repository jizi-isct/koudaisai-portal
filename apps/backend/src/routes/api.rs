mod approval_requests;
mod document_categories;
mod documents;
mod files;
mod forms;
mod groups;
mod notifications;
mod plans_info;
mod users;
mod util;

use crate::routes::AppState;
use axum::{Router, ServiceExt};
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "init /api")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/v2/forms", forms::init_router())
        .nest("/v2/documents", documents::init_router())
        .nest(
            "/v2/document-categories",
            document_categories::init_router(),
        )
        .nest("/v2/files", files::init_router())
        .nest("/v2/users", users::init_router())
        .nest("/v2/notifications", notifications::init_router())
        .nest("/v2/util", util::init_router())
        .nest("/v2/approval-requests", approval_requests::init_router())
        .nest("/v2/groups", groups::init_router())
        .nest_service("/plans_info", plans_info::init_service())
}
