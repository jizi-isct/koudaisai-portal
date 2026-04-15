mod approval_requests;

use crate::application::user::{
    GetUserNotificationsError, get_user_notifications_with_read_status,
};
use crate::entities::user::{UserRead, UserUpdate};
use crate::entities::user_id::UserId;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "init /api/v2/users/{user_id}")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_user).patch(patch_user))
        .route("/notifications", get(get_notifications))
        .nest("/approval_requests", approval_requests::init_router())
        .route("/m_address", post(post_m_address))
}

#[instrument(name = "GET /api/v2/users/:user_id", skip(state, current_user))]
async fn get_user(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<UserId>,
) -> AppResponse {
    match current_user {
        CurrentUser::None => {
            // 非ログイン時: ユーザー情報は取得不可
            Ok((StatusCode::FORBIDDEN, ().into_response()))
        }
        CurrentUser::User(claims) => {
            // 一般ユーザーの場合: ユーザーが自身の所属する参加団体に所属している場合のみ取得可能
            let current_user = UserRead::from_claims(claims, &state.db_conn).await?;

            let user = match user_id {
                UserId::Uuid(uuid) => UserRead::find_from_id(uuid, &state.db_conn).await?,
                UserId::Me => Some(current_user.clone()),
            };

            match user {
                Some(user) => {
                    if user.group_id != current_user.group_id {
                        return Ok((StatusCode::NOT_FOUND, ().into_response()));
                    }
                    Ok((StatusCode::OK, Json(user).into_response()))
                }
                None => Ok((StatusCode::NOT_FOUND, ().into_response())),
            }
        }
        CurrentUser::Admin(_) => {
            // 管理者の場合: ユーザー情報を取得(me非対応)
            let user = match user_id {
                UserId::Uuid(uuid) => UserRead::find_from_id(uuid, &state.db_conn).await?,
                UserId::Me => None,
            };

            match user {
                Some(user) => Ok((StatusCode::OK, Json(user).into_response())),
                None => Ok((StatusCode::NOT_FOUND, ().into_response())),
            }
        }
    }
}

#[instrument(name = "PATCH /api/v2/users/:user_id", skip(state, current_user))]
async fn patch_user(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<Uuid>,
    Json(user_update): Json<UserUpdate>,
) -> AppResponse {
    // 非ログイン時: ユーザー情報は更新不可
    if let CurrentUser::None = current_user {
        return Ok((StatusCode::FORBIDDEN, ().into_response()));
    }

    // 一般ユーザーの場合: ユーザー情報は更新不可
    if let CurrentUser::User(_) = current_user {
        return Ok((StatusCode::FORBIDDEN, ().into_response()));
    }

    // 管理者の場合: ユーザー情報を更新
    user_update.update(user_id, &state.db_conn).await?;
    Ok((StatusCode::NO_CONTENT, ().into_response()))
}

#[instrument(
    name = "GET /api/v2/users/{user_id}/notifications",
    skip(state, current_user)
)]
async fn get_notifications(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<UserId>,
) -> AppResponse {
    let notifications =
        match get_user_notifications_with_read_status(&current_user, user_id, &state.db_conn).await
        {
            Ok(notifications) => notifications,
            Err(GetUserNotificationsError::Forbidden) => {
                return Ok((StatusCode::FORBIDDEN, ().into_response()));
            }
            Err(GetUserNotificationsError::NotFound) => {
                return Ok((StatusCode::NOT_FOUND, ().into_response()));
            }
            Err(GetUserNotificationsError::Internal(e)) => return Err(e.into()),
        };

    Ok((StatusCode::OK, Json(notifications).into_response()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostMAddressPayload {
    m_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostMAddressResponse {
    activation_token: String,
}
#[instrument(name = "POST /api/v2/users/{user_id}/m_address", skip(state))]
async fn post_m_address(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<UserId>,
    Json(payload): Json<PostMAddressPayload>,
) -> AppResponse {
    if let CurrentUser::Admin(_) = current_user {
        // ユーザー情報
        let user = match user_id {
            UserId::Uuid(uuid) => UserRead::find_from_id(uuid, &state.db_conn).await?,
            UserId::Me => return Ok((StatusCode::FORBIDDEN, ().into_response())),
        };
        let user = match user {
            Some(user) => user,
            None => return Ok((StatusCode::NOT_FOUND, ().into_response())),
        };

        // mアドレスの更新
        let activation_token = user
            .change_m_address(
                &state.db_conn,
                payload.m_address.clone(),
                state.web.auth.activation_salt.clone(),
                state.web.auth.stretch_cost.clone() as i32,
            )
            .await?;

        Ok((
            StatusCode::OK,
            Json(PostMAddressResponse { activation_token }).into_response(),
        ))
    } else {
        Ok((StatusCode::FORBIDDEN, ().into_response()))
    }
}
