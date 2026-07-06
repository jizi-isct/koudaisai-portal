use super::super::ErrorMessage;
use super::super::V3State;
use super::dto::{FormCreate, FormRead, FormUpdate};
use crate::application::error::{ApplicationOperationError, DeleteError, UpdateError};
use crate::domain::actor_ctx::ActorContext;
use crate::domain::form::FormType as DomainFormType;
use crate::domain::form_id::FormId;
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
    InternalServerError(ErrorMessage),
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
        Err(_) => GetFormsResponse::InternalServerError(ErrorMessage::internal_server_error()),
    }
}

#[http_response]
pub enum PostFormResponse {
    #[response(status = CREATED)]
    Created(FormRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid form")]
    UnprocessableEntity(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
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
            _ => PostFormResponse::InternalServerError(ErrorMessage::internal_server_error()),
        },
        Err(ApplicationOperationError::Unauthorized) => {
            PostFormResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::InvalidInput(_)) => {
            PostFormResponse::UnprocessableEntity(ErrorMessage::unprocessable_entity())
        }
        Err(_) => PostFormResponse::InternalServerError(ErrorMessage::internal_server_error()),
    }
}

#[http_response]
pub enum GetFormResponse {
    #[response(status = OK, description = "Form found")]
    Ok(FormRead),
    #[response(status = NOT_FOUND, description = "Form not found")]
    NotFound(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
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
        Ok(None) => GetFormResponse::NotFound(ErrorMessage::not_found()),
        Err(_) => GetFormResponse::InternalServerError(ErrorMessage::internal_server_error()),
    }
}

#[http_response]
pub enum PatchFormResponse {
    #[response(status = OK, description = "Form updated")]
    Ok(FormRead),
    #[response(status = NOT_FOUND, description = "Form not found")]
    NotFound(ErrorMessage),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid form")]
    UnprocessableEntity(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
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
            _ => PatchFormResponse::InternalServerError(ErrorMessage::internal_server_error()),
        },
        Err(ApplicationOperationError::Unauthorized) => {
            PatchFormResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PatchFormResponse::NotFound(ErrorMessage::not_found())
        }
        Err(ApplicationOperationError::InvalidInput(_)) => {
            PatchFormResponse::UnprocessableEntity(ErrorMessage::unprocessable_entity())
        }
        Err(_) => PatchFormResponse::InternalServerError(ErrorMessage::internal_server_error()),
    }
}

#[http_response]
pub enum DeleteFormResponse {
    #[response(status = NO_CONTENT, description = "Form deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Form not found")]
    NotFound(ErrorMessage),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
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
        Err(ApplicationOperationError::Unauthorized) => {
            DeleteFormResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteFormResponse::NotFound(ErrorMessage::not_found())
        }
        Err(_) => DeleteFormResponse::InternalServerError(ErrorMessage::internal_server_error()),
    }
}
