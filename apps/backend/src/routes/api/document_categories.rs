use crate::entities::document_category;
use crate::entities::document_category::{DocumentCategoryRead, DocumentCategoryWrite};
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
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
        .route(
            "/{category_id}",
            get(get_document_category)
                .patch(patch_document_category)
                .delete(delete_document_category),
        )
}

#[instrument(name = "GET /api/v1/document-categories", skip(state))]
async fn get_document_categories(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> AppResponse {
    let mut document_categories = DocumentCategoryRead::get_all(&state.db_conn).await?;

    document_categories.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    Ok((StatusCode::OK, Json(document_categories).into_response()))
}

#[instrument(name = "POST /api/v1/document-categories", skip(state))]
async fn post_document_categories(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(document_category): Json<DocumentCategoryWrite>,
) -> AppResponse {
    if let CurrentUser::Admin(..) = current_user {
        let document_category = document_category
            .insert(Uuid::new_v4(), &state.db_conn)
            .await?;

        Ok((StatusCode::OK, Json(document_category).into_response()))
    } else {
        Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()))
    }
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

#[instrument(name = "PATCH /api/v1/document-categories/{category_id}", skip(state))]
async fn patch_document_category(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(category_id): Path<Uuid>,
    Json(document_category): Json<DocumentCategoryWrite>,
) -> AppResponse {
    if let CurrentUser::Admin(..) = current_user {
        match document_category
            .update(category_id, &state.db_conn)
            .await?
        {
            Some(document_category) => {
                Ok((StatusCode::OK, Json(document_category).into_response()))
            }
            None => Ok((StatusCode::NOT_FOUND, "Not found.".into_response())),
        }
    } else {
        Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()))
    }
}

#[instrument(name = "DELETE /api/v1/document-categories/{category_id}", skip(state))]
async fn delete_document_category(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(category_id): Path<Uuid>,
) -> AppResponse {
    if let CurrentUser::Admin(..) = current_user {
        if document_category::delete_document_category(category_id, &state.db_conn).await? > 0 {
            Ok((StatusCode::OK, "Deleted.".into_response()))
        } else {
            Ok((StatusCode::NOT_FOUND, "Not found.".into_response()))
        }
    } else {
        Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()))
    }
}
