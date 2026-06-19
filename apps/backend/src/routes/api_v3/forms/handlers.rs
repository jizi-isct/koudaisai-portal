use super::super::V3State;
use super::dto::{FormCreate, FormRead, FormUpdate};
use crate::application::error::{ApplicationOperationError, DeleteError, UpdateError};
use crate::domain::form::FormType as DomainFormType;
use crate::domain::form_id::FormId;
use crate::domain::actor_ctx::ActorContext;
use axum::Json;
use axum::extract::{Path, State};
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
pub async fn get_forms(State(st): State<V3State>, actor: ActorContext) -> GetFormsResponse {
    match st.app.form().get_all(&actor).await {
        Ok(forms) => GetFormsResponse::Ok(forms.iter().map(FormRead::from).collect()),
        Err(_) => GetFormsResponse::InternalServerError,
    }
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
pub async fn post_form(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<FormCreate>,
) -> PostFormResponse {
    let dtype: DomainFormType = (&body.r#type).into();
    match st
        .app
        .form()
        .create(
            &actor,
            body.targets.clone(),
            body.name.clone(),
            body.summary.clone(),
            body.due_date,
            dtype,
        )
        .await
    {
        Ok(form_id) => match st.app.form().get_by_id(&actor, form_id).await {
            Ok(Some(f)) => PostFormResponse::Created(FormRead::from(&f)),
            _ => PostFormResponse::InternalServerError,
        },
        Err(ApplicationOperationError::Unauthorized) => PostFormResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(_)) => PostFormResponse::UnprocessableEntity,
        Err(_) => PostFormResponse::InternalServerError,
    }
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
pub async fn get_form(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<FormPath>,
) -> GetFormResponse {
    match st.app.form().get_by_id(&actor, FormId::new(path.id)).await {
        Ok(Some(f)) => GetFormResponse::Ok(FormRead::from(&f)),
        Ok(None) => GetFormResponse::NotFound,
        Err(_) => GetFormResponse::InternalServerError,
    }
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
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<FormPath>,
    Json(body): Json<FormUpdate>,
) -> PatchFormResponse {
    let form_id = FormId::new(path.id);
    match st
        .app
        .form()
        .update(
            &actor,
            form_id,
            body.targets,
            body.name,
            body.summary,
            body.due_date,
            None,
        )
        .await
    {
        Ok(()) => match st.app.form().get_by_id(&actor, form_id).await {
            Ok(Some(f)) => PatchFormResponse::Ok(FormRead::from(&f)),
            _ => PatchFormResponse::InternalServerError,
        },
        Err(ApplicationOperationError::Unauthorized) => PatchFormResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PatchFormResponse::NotFound
        }
        Err(ApplicationOperationError::InvalidInput(_)) => PatchFormResponse::UnprocessableEntity,
        Err(_) => PatchFormResponse::InternalServerError,
    }
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
pub async fn delete_form(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<FormPath>,
) -> DeleteFormResponse {
    match st.app.form().delete(&actor, FormId::new(path.id)).await {
        Ok(()) => DeleteFormResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteFormResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteFormResponse::NotFound
        }
        Err(_) => DeleteFormResponse::InternalServerError,
    }
}
