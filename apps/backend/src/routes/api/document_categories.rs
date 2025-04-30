use crate::entities::document_category::DocumentCategoryRead;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use http::StatusCode;
use sea_orm::ActiveModelTrait;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "init /api/v1/document-categories")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_document_categories))
}

#[instrument(name = "GET /api/v1/document-categories", skip(state))]
async fn get_document_categories(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> AppResponse {
    let document_categories = DocumentCategoryRead::get_all(&state.db_conn).await?;

    Ok((StatusCode::OK, Json(document_categories).into_response()))
}
