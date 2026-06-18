use super::dto::{FormCreate, FormRead, FormUpdate};
use axum::Json;
use axum::extract::Path;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct FormPath {
    id: Uuid,
}

#[http_response]
pub enum GetFormsResponse {
    #[response(status = OK)]
    Ok(Vec<FormRead>),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get all forms visible to the caller.",
    path = "/",
    responses(GetFormsResponse),
    tag = super::super::FORMS_TAG
)]
pub async fn get_forms() -> GetFormsResponse {
    todo!()
}

#[http_response]
pub enum PostFormResponse {
    #[response(status = CREATED)]
    Created(FormRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid form")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    post,
    description = "Create a form.",
    path = "/",
    responses(PostFormResponse),
    request_body = FormCreate,
    tag = super::super::FORMS_TAG
)]
pub async fn post_form(Json(body): Json<FormCreate>) -> PostFormResponse {
    todo!()
}

#[http_response]
pub enum GetFormResponse {
    #[response(status = OK, description = "Form found")]
    Ok(FormRead),
    #[response(status = NOT_FOUND, description = "Form not found")]
    NotFound,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get a form by id.",
    params(FormPath),
    path = "/{id}",
    responses(GetFormResponse),
    tag = super::super::FORMS_TAG
)]
pub async fn get_form(Path(path): Path<FormPath>) -> GetFormResponse {
    todo!()
}

#[http_response]
pub enum PatchFormResponse {
    #[response(status = OK, description = "Form updated")]
    Ok(FormRead),
    #[response(status = NOT_FOUND, description = "Form not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid form")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    patch,
    description = "Edit a form by id.",
    params(FormPath),
    path = "/{id}",
    responses(PatchFormResponse),
    request_body = FormUpdate,
    tag = super::super::FORMS_TAG
)]
pub async fn patch_form(
    Path(path): Path<FormPath>,
    Json(body): Json<FormUpdate>,
) -> PatchFormResponse {
    todo!()
}

#[http_response]
pub enum DeleteFormResponse {
    #[response(status = NO_CONTENT, description = "Form deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Form not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Delete a form by id.",
    params(FormPath),
    path = "/{id}",
    responses(DeleteFormResponse),
    tag = super::super::FORMS_TAG
)]
pub async fn delete_form(Path(path): Path<FormPath>) -> DeleteFormResponse {
    todo!()
}
