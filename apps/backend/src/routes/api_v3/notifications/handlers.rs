use super::dto::{NotificationCreate, NotificationRead, NotificationUpdate};
use axum::Json;
use axum::extract::Path;
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
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get all notifications visible to the caller.",
    path = "/",
    responses(GetNotificationsResponse),
    tag = super::super::NOTIFICATIONS_TAG
)]
pub async fn get_notifications() -> GetNotificationsResponse {
    todo!()
}

#[http_response]
pub enum PostNotificationResponse {
    #[response(status = CREATED)]
    Created(NotificationRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid notification")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    post,
    description = "Create a notification.",
    path = "/",
    responses(PostNotificationResponse),
    request_body = NotificationCreate,
    tag = super::super::NOTIFICATIONS_TAG
)]
pub async fn post_notification(Json(body): Json<NotificationCreate>) -> PostNotificationResponse {
    todo!()
}

#[http_response]
pub enum GetNotificationResponse {
    #[response(status = OK, description = "Notification found")]
    Ok(NotificationRead),
    #[response(status = NOT_FOUND, description = "Notification not found")]
    NotFound,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get a notification by id.",
    params(NotificationPath),
    path = "/{id}",
    responses(GetNotificationResponse),
    tag = super::super::NOTIFICATIONS_TAG
)]
pub async fn get_notification(Path(path): Path<NotificationPath>) -> GetNotificationResponse {
    todo!()
}

#[http_response]
pub enum PatchNotificationResponse {
    #[response(status = OK, description = "Notification updated")]
    Ok(NotificationRead),
    #[response(status = NOT_FOUND, description = "Notification not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid notification")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
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
    Path(path): Path<NotificationPath>,
    Json(body): Json<NotificationUpdate>,
) -> PatchNotificationResponse {
    todo!()
}

#[http_response]
pub enum DeleteNotificationResponse {
    #[response(status = NO_CONTENT, description = "Notification deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Notification not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Delete a notification by id.",
    params(NotificationPath),
    path = "/{id}",
    responses(DeleteNotificationResponse),
    tag = super::super::NOTIFICATIONS_TAG
)]
pub async fn delete_notification(Path(path): Path<NotificationPath>) -> DeleteNotificationResponse {
    todo!()
}
