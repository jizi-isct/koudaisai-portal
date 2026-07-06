use super::super::ErrorMessage;
use super::super::V3State;
use super::dto::{NotificationCreate, NotificationRead, NotificationUpdate};
use crate::application::error::{ApplicationOperationError, DeleteError, UpdateError};
use crate::domain::actor_ctx::ActorContext;
use crate::domain::notification_id::NotificationId;
use axum::Json;
use axum::extract::{Path, State};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct NotificationPath {
    id: Uuid,
}

#[http_response]
pub enum GetNotificationsResponse {
    #[response(status = OK)]
    Ok(Vec<NotificationRead>),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    get,
    description = "Get all notifications visible to the caller.",
    path = "/",
    responses(GetNotificationsResponse),
    tag = super::super::NOTIFICATIONS_TAG
)]
pub async fn get_notifications(
    State(st): State<V3State>,
    actor: ActorContext,
) -> GetNotificationsResponse {
    match st.app.notification().get_all(&actor).await {
        Ok(notifications) => {
            GetNotificationsResponse::Ok(notifications.iter().map(NotificationRead::from).collect())
        }
        Err(_) => {
            GetNotificationsResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}

#[http_response]
pub enum PostNotificationResponse {
    #[response(status = CREATED)]
    Created(NotificationRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid notification")]
    UnprocessableEntity(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    post,
    description = "Create a notification.",
    path = "/",
    responses(PostNotificationResponse),
    request_body = NotificationCreate,
    tag = super::super::NOTIFICATIONS_TAG
)]
pub async fn post_notification(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<NotificationCreate>,
) -> PostNotificationResponse {
    let ntype = match body.r#type.to_domain() {
        Ok(t) => t,
        Err(_) => {
            return PostNotificationResponse::UnprocessableEntity(
                ErrorMessage::unprocessable_entity(),
            );
        }
    };
    match st
        .app
        .notification()
        .create(&actor, body.targets, ntype)
        .await
    {
        Ok(id) => match st.app.notification().get_by_id(&actor, id).await {
            Ok(Some(n)) => PostNotificationResponse::Created(NotificationRead::from(&n)),
            _ => {
                PostNotificationResponse::InternalServerError(ErrorMessage::internal_server_error())
            }
        },
        Err(ApplicationOperationError::Unauthorized) => {
            PostNotificationResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::InvalidInput(_)) => {
            PostNotificationResponse::UnprocessableEntity(ErrorMessage::unprocessable_entity())
        }
        Err(_) => {
            PostNotificationResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}

#[http_response]
pub enum GetNotificationResponse {
    #[response(status = OK, description = "Notification found")]
    Ok(NotificationRead),
    #[response(status = NOT_FOUND, description = "Notification not found")]
    NotFound(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    get,
    description = "Get a notification by id.",
    params(NotificationPath),
    path = "/{id}",
    responses(GetNotificationResponse),
    tag = super::super::NOTIFICATIONS_TAG
)]
pub async fn get_notification(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<NotificationPath>,
) -> GetNotificationResponse {
    match st
        .app
        .notification()
        .get_by_id(&actor, NotificationId::new(path.id))
        .await
    {
        Ok(Some(n)) => GetNotificationResponse::Ok(NotificationRead::from(&n)),
        Ok(None) => GetNotificationResponse::NotFound(ErrorMessage::not_found()),
        Err(_) => {
            GetNotificationResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}

#[http_response]
pub enum PatchNotificationResponse {
    #[response(status = OK, description = "Notification updated")]
    Ok(NotificationRead),
    #[response(status = NOT_FOUND, description = "Notification not found")]
    NotFound(ErrorMessage),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid notification")]
    UnprocessableEntity(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    patch,
    description = "Edit a notification by id.",
    params(NotificationPath),
    path = "/{id}",
    responses(PatchNotificationResponse),
    request_body = NotificationUpdate,
    tag = super::super::NOTIFICATIONS_TAG
)]
pub async fn patch_notification(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<NotificationPath>,
    Json(body): Json<NotificationUpdate>,
) -> PatchNotificationResponse {
    let markdown = match (body.title, body.content) {
        (Some(t), Some(c)) => Some((t, c)),
        _ => None,
    };
    match st
        .app
        .notification()
        .update(&actor, NotificationId::new(path.id), body.targets, markdown)
        .await
    {
        Ok(n) => PatchNotificationResponse::Ok(NotificationRead::from(&n)),
        Err(ApplicationOperationError::Unauthorized) => {
            PatchNotificationResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PatchNotificationResponse::NotFound(ErrorMessage::not_found())
        }
        Err(ApplicationOperationError::InvalidInput(_)) => {
            PatchNotificationResponse::UnprocessableEntity(ErrorMessage::unprocessable_entity())
        }
        Err(_) => {
            PatchNotificationResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}

#[http_response]
pub enum DeleteNotificationResponse {
    #[response(status = NO_CONTENT, description = "Notification deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Notification not found")]
    NotFound(ErrorMessage),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    delete,
    description = "Delete a notification by id.",
    params(NotificationPath),
    path = "/{id}",
    responses(DeleteNotificationResponse),
    tag = super::super::NOTIFICATIONS_TAG
)]
pub async fn delete_notification(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<NotificationPath>,
) -> DeleteNotificationResponse {
    match st
        .app
        .notification()
        .delete(&actor, NotificationId::new(path.id))
        .await
    {
        Ok(()) => DeleteNotificationResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => {
            DeleteNotificationResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteNotificationResponse::NotFound(ErrorMessage::not_found())
        }
        Err(_) => {
            DeleteNotificationResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}
