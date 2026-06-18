use super::dto::{DocumentCreate, DocumentRead, DocumentUpdate, DocumentsByCategoryEntry};
use axum::Json;
use axum::extract::{Path, Query};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct DocumentPath {
    id: Uuid,
}

#[derive(Deserialize, IntoParams)]
pub struct DocumentQuery {
    category: Option<Uuid>,
}

#[http_response]
pub enum GetDocumentsResponse {
    #[response(status = OK)]
    Ok(Vec<DocumentRead>),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get all documents visible to the caller.",
    params(DocumentQuery),
    path = "/",
    responses(GetDocumentsResponse),
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn get_documents(Query(query): Query<DocumentQuery>) -> GetDocumentsResponse {
    todo!()
}

/// by-category のクエリパラメーター。
#[derive(Deserialize, IntoParams)]
pub struct DocumentsByCategoryQuery {
    /// 真のとき、ドキュメントが無いカテゴリも空リストで含める。
    include_empty_categories: Option<bool>,
}

#[http_response]
pub enum GetDocumentsByCategoryResponse {
    #[response(status = OK)]
    Ok(Vec<DocumentsByCategoryEntry>),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get documents visible to the caller, grouped by category.",
    params(DocumentsByCategoryQuery),
    path = "/by-category",
    responses(GetDocumentsByCategoryResponse),
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn get_documents_by_category(
    Query(query): Query<DocumentsByCategoryQuery>,
) -> GetDocumentsByCategoryResponse {
    todo!()
}

#[http_response]
pub enum PostDocumentResponse {
    #[response(status = CREATED)]
    Created(DocumentRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid document")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    post,
    description = "Create a document.",
    path = "/",
    responses(PostDocumentResponse),
    request_body = DocumentCreate,
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn post_document(Json(body): Json<DocumentCreate>) -> PostDocumentResponse {
    todo!()
}

#[http_response]
pub enum GetDocumentResponse {
    #[response(status = OK, description = "Document found")]
    Ok(DocumentRead),
    #[response(status = NOT_FOUND, description = "Document not found")]
    NotFound,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get a document by id.",
    params(DocumentPath),
    path = "/{id}",
    responses(GetDocumentResponse),
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn get_document(Path(path): Path<DocumentPath>) -> GetDocumentResponse {
    todo!()
}

#[http_response]
pub enum PatchDocumentResponse {
    #[response(status = OK, description = "Document updated")]
    Ok(DocumentRead),
    #[response(status = NOT_FOUND, description = "Document not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid document")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    patch,
    description = "Edit a document by id.",
    params(DocumentPath),
    path = "/{id}",
    responses(PatchDocumentResponse),
    request_body = DocumentUpdate,
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn patch_document(
    Path(path): Path<DocumentPath>,
    Json(body): Json<DocumentUpdate>,
) -> PatchDocumentResponse {
    todo!()
}

#[http_response]
pub enum DeleteDocumentResponse {
    #[response(status = NO_CONTENT, description = "Document deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Document not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Delete a document by id.",
    params(DocumentPath),
    path = "/{id}",
    responses(DeleteDocumentResponse),
    tag = super::super::DOCUMENTS_TAG
)]
pub async fn delete_document(Path(path): Path<DocumentPath>) -> DeleteDocumentResponse {
    todo!()
}
