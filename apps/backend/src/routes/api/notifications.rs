use crate::entities::notification;
use crate::entities::notification::{NotificationCreate, NotificationRead, NotificationUpdate};
use crate::entities::user::UserRead;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::StatusCode;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "init /api/v2/notifications")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_notifications).post(post_notifications))
        .route(
            "/{notification_id}",
            get(get_notification)
                .patch(patch_notification)
                .delete(delete_notification),
        )
}

#[instrument(name = "GET /api/v2/notifications", skip(state, current_user))]
async fn get_notifications(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    if let CurrentUser::Admin(_) = current_user {
        let notifications = NotificationRead::get_all(&state.db_conn).await?;
        Ok((StatusCode::OK, Json(notifications).into_response()))
    } else {
        Ok((StatusCode::FORBIDDEN, ().into_response()))
    }
}

#[instrument(name = "POST /api/v2/notifications", skip(state, current_user))]
async fn post_notifications(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(notification): Json<NotificationCreate>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(claims) => {
            let notification = NotificationCreate::insert(
                notification,
                &state.db_conn,
                Some(Uuid::from_str(claims.subject().as_str())?),
            )
            .await?;
            Ok((StatusCode::CREATED, Json(notification).into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    }
}

#[instrument(
    name = "GET /api/v2/notifications/{notification_id}",
    skip(state, current_user)
)]
async fn get_notification(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(notification_id): Path<Uuid>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            if let Some(notification) =
                NotificationRead::find_by_id(&state.db_conn, notification_id).await?
            {
                Ok((StatusCode::OK, Json(notification).into_response()))
            } else {
                Ok((StatusCode::NOT_FOUND, ().into_response()))
            }
        }
        CurrentUser::User(claims) => {
            let user = UserRead::from_claims(claims, &state.db_conn).await?;
            if let Some(notification) =
                NotificationRead::find_by_id(&state.db_conn, notification_id).await?
            {
                for target_specifier in &notification.target {
                    if target_specifier
                        .does_user_match(Some(&user), &state.db_conn)
                        .await?
                    {
                        return Ok((StatusCode::OK, Json(notification).into_response()));
                    }
                }
            }
            Ok((StatusCode::NOT_FOUND, ().into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    }
}

#[instrument(
    name = "PATCH /api/v2/notifications/{notification_id}",
    skip(state, current_user)
)]
async fn patch_notification(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(notification_id): Path<Uuid>,
    Json(notification): Json<NotificationUpdate>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(claims) => {
            let updated_notification = notification
                .update(
                    &state.db_conn,
                    notification_id,
                    Some(Uuid::from_str(claims.subject().as_str())?),
                )
                .await?;
            Ok((StatusCode::OK, Json(updated_notification).into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    }
}

#[instrument(
    name = "DELETE /api/v2/notifications/{notification_id}",
    skip(state, current_user)
)]
async fn delete_notification(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(notification_id): Path<Uuid>,
) -> AppResponse {
    if let CurrentUser::Admin(_) = current_user {
        notification::delete_notification(&state.db_conn, notification_id).await?;
        Ok((StatusCode::NO_CONTENT, ().into_response()))
    } else {
        Ok((StatusCode::FORBIDDEN, ().into_response()))
    }
}
