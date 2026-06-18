use super::dto::{
    MAddressUpdate, MAddressUpdated, UserCreate, UserCreated, UserNotificationRead, UserRead,
    UserUpdate,
};
use axum::Json;
use axum::extract::Path;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;
use uuid::Uuid;

#[http_response]
pub enum GetUsersResponse {
    #[response(status = OK)]
    Ok(Vec<UserRead>),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get all users.",
    path = "/",
    responses(GetUsersResponse),
    tag = super::super::USERS_TAG
)]
pub async fn get_users() -> GetUsersResponse {
    todo!()
}

/// ユーザーのパスパラメーター
#[derive(Deserialize, IntoParams)]
pub struct UserPath {
    id: Uuid,
}

#[http_response]
pub enum PutUserResponse {
    #[response(status = CREATED, description = "User created; returns the activation token")]
    Created(UserCreated),
    #[response(status = OK, description = "User replaced")]
    Ok(UserRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = CONFLICT, description = "Conflict")]
    Conflict,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    put,
    description = "Create a new user or replace an existing one.",
    path = "/{id}",
    params(UserPath),
    responses(PutUserResponse),
    request_body = UserCreate,
    tag = super::super::USERS_TAG
)]
pub async fn put_user(Path(path): Path<UserPath>, Json(user): Json<UserCreate>) -> PutUserResponse {
    todo!()
}

#[http_response]
pub enum GetUserResponse {
    #[response(status = OK, description = "User found")]
    Ok(UserRead),
    #[response(status = NOT_FOUND, description = "User not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get a user by id.",
    path = "/{id}",
    params(UserPath),
    responses(GetUserResponse),
    tag = super::super::USERS_TAG
)]
pub async fn get_user(Path(path): Path<UserPath>) -> GetUserResponse {
    todo!()
}

#[http_response]
pub enum PatchUserResponse {
    #[response(status = OK, description = "User updated")]
    Ok(UserRead),
    #[response(status = NOT_FOUND, description = "User not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    patch,
    description = "Edit a user by id.",
    path = "/{id}",
    params(UserPath),
    responses(PatchUserResponse),
    request_body=UserUpdate,
    tag = super::super::USERS_TAG
)]
pub async fn patch_user(
    Path(path): Path<UserPath>,
    Json(user): Json<UserUpdate>,
) -> PatchUserResponse {
    todo!()
}

#[http_response]
pub enum DeleteUserResponse {
    #[response(status = NO_CONTENT, description = "User deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "User not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Delete a user by id.",
    path = "/{id}",
    params(UserPath),
    responses(DeleteUserResponse),
    tag = super::super::USERS_TAG
)]
pub async fn delete_user(Path(path): Path<UserPath>) -> DeleteUserResponse {
    todo!()
}

#[http_response]
pub enum GetUserNotificationsResponse {
    #[response(status = OK)]
    Ok(Vec<UserNotificationRead>),
    #[response(status = NOT_FOUND, description = "User not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get a user's notifications with per-user read state.",
    path = "/{id}/notifications",
    params(UserPath),
    responses(GetUserNotificationsResponse),
    tag = super::super::USERS_TAG
)]
pub async fn get_user_notifications(Path(path): Path<UserPath>) -> GetUserNotificationsResponse {
    todo!()
}

#[http_response]
pub enum PostUserMAddressResponse {
    #[response(status = OK, description = "m_address changed; returns a new activation token")]
    Ok(MAddressUpdated),
    #[response(status = NOT_FOUND, description = "User not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    post,
    description = "Change a user's m_address and re-issue an activation token.",
    path = "/{id}/m_address",
    params(UserPath),
    responses(PostUserMAddressResponse),
    request_body = MAddressUpdate,
    tag = super::super::USERS_TAG
)]
pub async fn post_user_m_address(
    Path(path): Path<UserPath>,
    Json(body): Json<MAddressUpdate>,
) -> PostUserMAddressResponse {
    todo!()
}
