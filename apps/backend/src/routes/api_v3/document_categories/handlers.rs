use super::super::V3State;
use super::dto::{DocumentCategoryCreate, DocumentCategoryRead, DocumentCategoryUpdate};
use crate::application::error::{ApplicationOperationError, DeleteError, UpdateError};
use crate::domain::actor_ctx::ActorContext;
use axum::Json;
use axum::extract::{Path, State};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct DocumentCategoryPath {
    id: Uuid,
}

#[http_response]
pub enum GetDocumentCategoriesResponse {
    #[response(status = OK)]
    Ok(Vec<DocumentCategoryRead>),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get all document categories.",
    path = "/",
    responses(GetDocumentCategoriesResponse),
    tag = super::super::DOCUMENT_CATEGORIES_TAG
)]
pub async fn get_document_categories(
    State(st): State<V3State>,
    actor: ActorContext,
) -> GetDocumentCategoriesResponse {
    match st.app.document_category().get_all(&actor).await {
        Ok(mut cats) => {
            cats.sort_by_key(|c| c.created_at());
            GetDocumentCategoriesResponse::Ok(
                cats.iter().map(DocumentCategoryRead::from).collect(),
            )
        }
        Err(ApplicationOperationError::Unauthorized) => GetDocumentCategoriesResponse::Forbidden,
        Err(_) => GetDocumentCategoriesResponse::InternalServerError,
    }
}

#[http_response]
pub enum GetDocumentCategoryResponse {
    #[response(status = OK, description = "Document category found")]
    Ok(DocumentCategoryRead),
    #[response(status = NOT_FOUND, description = "Document category not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get a document category by id.",
    params(DocumentCategoryPath),
    path = "/{id}",
    responses(GetDocumentCategoryResponse),
    tag = super::super::DOCUMENT_CATEGORIES_TAG
)]
pub async fn get_document_category(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<DocumentCategoryPath>,
) -> GetDocumentCategoryResponse {
    match st.app.document_category().get_by_id(&actor, path.id).await {
        Ok(Some(cat)) => GetDocumentCategoryResponse::Ok(DocumentCategoryRead::from(&cat)),
        Ok(None) => GetDocumentCategoryResponse::NotFound,
        Err(ApplicationOperationError::Unauthorized) => GetDocumentCategoryResponse::Forbidden,
        Err(_) => GetDocumentCategoryResponse::InternalServerError,
    }
}

#[http_response]
pub enum PostDocumentCategoryResponse {
    #[response(status = CREATED)]
    Created(DocumentCategoryRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid document category")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    post,
    description = "Create a document category.",
    path = "/",
    responses(PostDocumentCategoryResponse),
    request_body = DocumentCategoryCreate,
    tag = super::super::DOCUMENT_CATEGORIES_TAG
)]
pub async fn post_document_category(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<DocumentCategoryCreate>,
) -> PostDocumentCategoryResponse {
    match st
        .app
        .document_category()
        .create(&actor, body.title, body.emoji)
        .await
    {
        Ok(cat) => PostDocumentCategoryResponse::Created(DocumentCategoryRead::from(&cat)),
        Err(ApplicationOperationError::Unauthorized) => PostDocumentCategoryResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(_)) => {
            PostDocumentCategoryResponse::UnprocessableEntity
        }
        Err(_) => PostDocumentCategoryResponse::InternalServerError,
    }
}

#[http_response]
pub enum PatchDocumentCategoryResponse {
    #[response(status = OK, description = "Document category updated")]
    Ok(DocumentCategoryRead),
    #[response(status = NOT_FOUND, description = "Document category not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid document category")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    patch,
    description = "Edit a document category by id.",
    params(DocumentCategoryPath),
    path = "/{id}",
    responses(PatchDocumentCategoryResponse),
    request_body = DocumentCategoryUpdate,
    tag = super::super::DOCUMENT_CATEGORIES_TAG
)]
pub async fn patch_document_category(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<DocumentCategoryPath>,
    Json(body): Json<DocumentCategoryUpdate>,
) -> PatchDocumentCategoryResponse {
    match st
        .app
        .document_category()
        .update_document_category(&actor, path.id, body.title, body.emoji.map(Some))
        .await
    {
        Ok(()) => match st.app.document_category().get_by_id(&actor, path.id).await {
            Ok(Some(cat)) => PatchDocumentCategoryResponse::Ok(DocumentCategoryRead::from(&cat)),
            Ok(None) => PatchDocumentCategoryResponse::NotFound,
            _ => PatchDocumentCategoryResponse::InternalServerError,
        },
        Err(ApplicationOperationError::Unauthorized) => PatchDocumentCategoryResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(_)) => {
            PatchDocumentCategoryResponse::UnprocessableEntity
        }
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PatchDocumentCategoryResponse::NotFound
        }
        Err(_) => PatchDocumentCategoryResponse::InternalServerError,
    }
}

#[http_response]
pub enum DeleteDocumentCategoryResponse {
    #[response(status = NO_CONTENT, description = "Document category deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Document category not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Delete a document category by id.",
    params(DocumentCategoryPath),
    path = "/{id}",
    responses(DeleteDocumentCategoryResponse),
    tag = super::super::DOCUMENT_CATEGORIES_TAG
)]
pub async fn delete_document_category(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<DocumentCategoryPath>,
) -> DeleteDocumentCategoryResponse {
    match st
        .app
        .document_category()
        .delete_document_category(&actor, path.id)
        .await
    {
        Ok(()) => DeleteDocumentCategoryResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteDocumentCategoryResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteDocumentCategoryResponse::NotFound
        }
        Err(_) => DeleteDocumentCategoryResponse::InternalServerError,
    }
}
