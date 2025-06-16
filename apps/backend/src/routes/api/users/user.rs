use crate::entities::notification::NotificationRead;
use crate::entities::user::{UserRead, UserUpdate};
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::{contains_uuid, AppResponse};
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "init /api/v1/users/{user_id}")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_user).patch(patch_user))
        .route("/notifications", get(get_notifications))
}

#[instrument(name = "GET /api/v1/users/:user_id", skip(state, current_user))]
async fn get_user(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<Uuid>,
) -> AppResponse {
    // 非ログイン時: ユーザー情報は取得不可
    if let CurrentUser::None = current_user {
        return Ok((StatusCode::FORBIDDEN, ().into_response()));
    }

    // 一般ユーザーの場合: ユーザーが自身の所属する参加団体に所属している場合のみ取得可能
    if let CurrentUser::User(claims) = current_user {
        let current_user = UserRead::from_claims(claims, &state.db_conn).await?;
        let exhibitor_read = current_user.get_exhibitor_read(&state.db_conn).await?;
        if !contains_uuid(exhibitor_read.representatives, current_user.id) {
            return Ok((StatusCode::NOT_FOUND, ().into_response()));
        }
    }

    // ユーザー情報を取得
    match UserRead::find_from_id(user_id, &state.db_conn).await? {
        Some(user) => Ok((StatusCode::OK, Json(user).into_response())),
        None => Ok((StatusCode::NOT_FOUND, ().into_response())),
    }
}

#[instrument(name = "PATCH /api/v1/users/:user_id", skip(state, current_user))]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetNotificationsResponseEntry {
    is_read: bool,
    notification: NotificationRead,
}

#[instrument(
    name = "GET /api/v1/users/{user_id}/notifications",
    skip(state, current_user)
)]
async fn get_notifications(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<Uuid>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            // 管理者の場合: 全てのユーザーの通知を取得可能
            // ユーザー情報を取得
            let user = UserRead::find_from_id(user_id, &state.db_conn).await?;
            if user.is_none() {
                return Ok((StatusCode::NOT_FOUND, ().into_response()));
            }
            let user = user.unwrap();

            // ユーザーの既読情報を取得
            let notifications = NotificationRead::get_all(&state.db_conn).await?;
            let mut response_body = vec![];
            for notification in notifications {
                let is_read = user
                    .is_notification_read(notification.id, &state.db_conn)
                    .await?;
                let entry = GetNotificationsResponseEntry {
                    is_read,
                    notification,
                };
                response_body.push(entry);
            }

            // レスポンスを返す
            Ok((StatusCode::OK, Json(response_body).into_response()))
        }
        CurrentUser::User(claims) => {
            // 一般ユーザーの場合: ユーザーが自身の所属する参加団体に所属している場合のみ取得可能
            let current_user = UserRead::from(claims, &state.db_conn).await?;
            let exhibition = current_user.get_exhibitor_read(&state.db_conn).await?;
            if !contains_uuid(exhibition.representatives, user_id) {
                return Ok((StatusCode::NOT_FOUND, ().into_response()));
            }

            // ユーザー情報を取得
            let user = UserRead::find_from_id(user_id, &state.db_conn).await?;
            if user.is_none() {
                return Ok((StatusCode::NOT_FOUND, ().into_response()));
            }
            let user = user.unwrap();

            // ユーザーの通知を取得
            let mut notifications = vec![];
            // ユーザーがアクセス可能な通知を取得
            for notification in NotificationRead::get_all(&state.db_conn).await? {
                for target_specifier in &notification.target {
                    if !target_specifier
                        .does_user_match(Some(&user), &state.db_conn)
                        .await?
                    {
                        continue;
                    }
                }
                notifications.push(notification)
            }

            // ユーザーが既読かどうかを確認
            let mut response_body = vec![];
            for notification in notifications {
                let is_read = user
                    .is_notification_read(notification.id, &state.db_conn)
                    .await?;
                let entry = GetNotificationsResponseEntry {
                    is_read,
                    notification,
                };
                response_body.push(entry);
            }

            // レスポンスを返す
            Ok((StatusCode::OK, Json(response_body).into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    }
}
