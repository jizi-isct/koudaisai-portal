use super::dto::{DocumentCategoryCreate, DocumentCategoryRead, DocumentCategoryUpdate};
use axum::Json;
use axum::extract::Path;
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
pub async fn get_document_categories() -> GetDocumentCategoriesResponse {
    todo!()
}

#[http_response]
pub enum GetDocumentCategoryResponse {
    #[response(status = OK, description = "Document category found")]
    Ok(DocumentCategoryRead),
    #[response(status = NOT_FOUND, description = "Document category not found")]
    NotFound,
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
    Path(path): Path<DocumentCategoryPath>,
) -> GetDocumentCategoryResponse {
    todo!()
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
    Json(body): Json<DocumentCategoryCreate>,
) -> PostDocumentCategoryResponse {
    todo!()
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
    Path(path): Path<DocumentCategoryPath>,
    Json(body): Json<DocumentCategoryUpdate>,
) -> PatchDocumentCategoryResponse {
    todo!()
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
    Path(path): Path<DocumentCategoryPath>,
) -> DeleteDocumentCategoryResponse {
    todo!()
}
