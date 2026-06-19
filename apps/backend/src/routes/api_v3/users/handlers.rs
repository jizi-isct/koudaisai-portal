use super::super::notifications::NotificationRead;
use super::super::V3State;
use super::dto::{MAddressUpdate, MAddressUpdated, UserCreate, UserCreated, UserRead, UserUpdate};
use crate::application::error::{ApplicationOperationError, DeleteError, InsertError, UpdateError};
use crate::domain::actor_ctx::ActorContext;
use crate::domain::email_address::EmailAddress;
use crate::domain::user_id::UserId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use axum::extract::{Path, State};
use axum::Json;
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
pub async fn get_users(State(st): State<V3State>, actor: ActorContext) -> GetUsersResponse {
    match st.app.user().get_all(&actor).await {
        Ok(users) => GetUsersResponse::Ok(users.iter().map(UserRead::from).collect()),
        Err(ApplicationOperationError::Unauthorized) => GetUsersResponse::Forbidden,
        Err(_) => GetUsersResponse::InternalServerError,
    }
}

#[http_response]
pub enum PostUserResponse {
    #[response(status = CREATED, description = "User created; returns the activation token")]
    Created(UserCreated),
    #[response(status = BAD_REQUEST, description = "Invalid input")]
    BadRequest,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = CONFLICT, description = "m_address already in use")]
    Conflict,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    post,
    description = "Create a new user (id is generated server-side); returns the activation token.",
    path = "/",
    responses(PostUserResponse),
    request_body = UserCreate,
    tag = super::super::USERS_TAG
)]
pub async fn post_user(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<UserCreate>,
) -> PostUserResponse {
    let user_id = UserId::new(Uuid::new_v4());
    let Ok(m_address) = EmailAddress::new(body.m_address) else {
        return PostUserResponse::BadRequest;
    };
    match st
        .app
        .user()
        .register(&actor, user_id, body.name, m_address)
        .await
    {
        Ok(user) => {
            // 作成直後に activation トークンを発行して返す(このときのみ返却)。
            let tx = SqliteTransaction::new(st.pool.clone());
            match st
                .app
                .auth(st.auth_config.clone(), (*st.dummy_phc).clone())
                .issue_activation_token(tx, user_id)
                .await
            {
                Ok(token) => PostUserResponse::Created(UserCreated {
                    user: UserRead::from(&user),
                    activation_token: token,
                }),
                Err(_) => PostUserResponse::InternalServerError,
            }
        }
        Err(ApplicationOperationError::Unauthorized) => PostUserResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(InsertError::Conflict)) => {
            PostUserResponse::Conflict
        }
        Err(ApplicationOperationError::InvalidInput(_)) => PostUserResponse::BadRequest,
        Err(_) => PostUserResponse::InternalServerError,
    }
}

/// ユーザーのパスパラメーター
#[derive(Deserialize, IntoParams)]
pub struct UserPath {
    #[param(value_type = uuid::Uuid)]
    id: UserId,
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
pub async fn get_user(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<UserPath>,
) -> GetUserResponse {
    match st.app.user().get_by_id(&actor, path.id).await {
        Ok(Some(u)) => GetUserResponse::Ok(UserRead::from(&u)),
        Ok(None) => GetUserResponse::NotFound,
        Err(ApplicationOperationError::Unauthorized) => GetUserResponse::Forbidden,
        Err(_) => GetUserResponse::InternalServerError,
    }
}

#[utoipa::path(
    get,
    description = "Get the currently authenticated user.",
    path = "/me",
    responses(GetUserResponse),
    tag = super::super::USERS_TAG
)]
pub async fn get_user_me(State(st): State<V3State>, actor: ActorContext) -> GetUserResponse {
    let Some(user_id) = actor.user_id() else {
        return GetUserResponse::Forbidden;
    };
    match st.app.user().get_by_id(&actor, user_id).await {
        Ok(Some(u)) => GetUserResponse::Ok(UserRead::from(&u)),
        Ok(None) => GetUserResponse::NotFound,
        Err(ApplicationOperationError::Unauthorized) => GetUserResponse::Forbidden,
        Err(_) => GetUserResponse::InternalServerError,
    }
}

#[http_response]
pub enum PatchUserResponse {
    #[response(status = OK, description = "User updated")]
    Ok(UserRead),
    #[response(status = BAD_REQUEST, description = "Invalid input")]
    BadRequest,
    #[response(status = NOT_FOUND, description = "User not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    patch,
    description = "Edit a user's name by id (m_address is changed via its own endpoint).",
    path = "/{id}",
    params(UserPath),
    responses(PatchUserResponse),
    request_body=UserUpdate,
    tag = super::super::USERS_TAG
)]
pub async fn patch_user(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<UserPath>,
    Json(body): Json<UserUpdate>,
) -> PatchUserResponse {
    // m アドレスは変更不可(専用エンドポイント)。氏名のみ更新する。
    match st
        .app
        .user()
        .edit_user(&actor, path.id, body.name, None)
        .await
    {
        Ok(u) => PatchUserResponse::Ok(UserRead::from(&u)),
        Err(ApplicationOperationError::Unauthorized) => PatchUserResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(_)) => PatchUserResponse::BadRequest,
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PatchUserResponse::NotFound
        }
        Err(_) => PatchUserResponse::InternalServerError,
    }
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
pub async fn delete_user(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<UserPath>,
) -> DeleteUserResponse {
    match st.app.user().delete_user(&actor, path.id).await {
        Ok(()) => DeleteUserResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteUserResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteUserResponse::NotFound
        }
        Err(_) => DeleteUserResponse::InternalServerError,
    }
}

#[http_response]
pub enum GetUserReadNotificationsResponse {
    #[response(status = OK)]
    Ok(Vec<NotificationRead>),
    #[response(status = NOT_FOUND, description = "User not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get the notifications the user has marked as read.",
    path = "/{id}/read-notifications",
    params(UserPath),
    responses(GetUserReadNotificationsResponse),
    tag = super::super::USERS_TAG
)]
pub async fn get_user_read_notifications(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<UserPath>,
) -> GetUserReadNotificationsResponse {
    let user_id = path.id;
    // 対象ユーザーの存在確認(authz は介在しない)。
    match st.app.find_user_for_auth(user_id).await {
        Ok(Some(_)) => {}
        Ok(None) => return GetUserReadNotificationsResponse::NotFound,
        Err(_) => return GetUserReadNotificationsResponse::InternalServerError,
    }
    // 対象ユーザーの認可コンテキスト(所属が無ければ None)。
    let target_ctx = match st.app.build_actor_context(user_id).await {
        Ok(c) => c,
        Err(_) => return GetUserReadNotificationsResponse::InternalServerError,
    };
    match st
        .app
        .notification()
        .get_for_user(&actor, user_id, target_ctx.as_ref())
        .await
    {
        // 既読(is_read==true)の通知のみを返す。
        Ok(items) => GetUserReadNotificationsResponse::Ok(
            items
                .into_iter()
                .filter(|(_, is_read)| *is_read)
                .map(|(n, _)| NotificationRead::from(&n))
                .collect(),
        ),
        Err(ApplicationOperationError::Unauthorized) => GetUserReadNotificationsResponse::Forbidden,
        Err(_) => GetUserReadNotificationsResponse::InternalServerError,
    }
}

#[http_response]
pub enum PostUserMAddressResponse {
    #[response(status = OK, description = "m_address changed; returns a new activation token")]
    Ok(MAddressUpdated),
    #[response(status = BAD_REQUEST, description = "Invalid input")]
    BadRequest,
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
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<UserPath>,
    Json(body): Json<MAddressUpdate>,
) -> PostUserMAddressResponse {
    let user_id = path.id;
    let Ok(m_address) = EmailAddress::new(body.m_address) else {
        return PostUserMAddressResponse::BadRequest;
    };
    match st
        .app
        .user()
        .change_m_address(&actor, user_id, m_address)
        .await
    {
        Ok(()) => {
            // m アドレス変更に伴い activation トークンを再発行する。
            let tx = SqliteTransaction::new(st.pool.clone());
            match st
                .app
                .auth(st.auth_config.clone(), (*st.dummy_phc).clone())
                .issue_activation_token(tx, user_id)
                .await
            {
                Ok(token) => PostUserMAddressResponse::Ok(MAddressUpdated {
                    activation_token: token,
                }),
                Err(_) => PostUserMAddressResponse::InternalServerError,
            }
        }
        Err(ApplicationOperationError::Unauthorized) => PostUserMAddressResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PostUserMAddressResponse::NotFound
        }
        Err(_) => PostUserMAddressResponse::InternalServerError,
    }
}
