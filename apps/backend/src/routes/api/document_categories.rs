use crate::entities::document_category::{DocumentCategoryRead, DocumentCategoryWrite};
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use http::StatusCode;
use sea_orm::ActiveModelTrait;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "init /api/v1/document-categories")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(get_document_categories).post(post_document_categories),
        )
        .route("/{category_id}", get(get_document_category))
}

#[instrument(name = "GET /api/v1/document-categories", skip(state))]
async fn get_document_categories(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> AppResponse {
    let document_categories = DocumentCategoryRead::get_all(&state.db_conn).await?;

    Ok((StatusCode::OK, Json(document_categories).into_response()))
}

#[instrument(name = "POST /api/v1/document-categories", skip(state))]
async fn post_document_categories(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(document_category): Json<DocumentCategoryWrite>,
) -> AppResponse {
    let document_category = document_category
        .insert(Uuid::new_v4(), &state.db_conn)
        .await?;

    Ok((StatusCode::OK, Json(document_category).into_response()))
}

#[instrument(name = "GET /api/v1/document-categories/{category_id}", skip(state))]
async fn get_document_category(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<Uuid>,
) -> AppResponse {
    match DocumentCategoryRead::find_by_id(category_id, &state.db_conn).await? {
        Some(document_category) => Ok((StatusCode::OK, Json(document_category).into_response())),
        None => Ok((StatusCode::NOT_FOUND, "Not found.".into_response())),
    }
}
