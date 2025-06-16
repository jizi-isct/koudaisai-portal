use crate::entities::user::{UserRead, UserUpdate};
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::{contains_uuid, AppResponse};
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::StatusCode;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "init /api/v1/users")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        .route("/:user_id", get(get_user))
}

#[instrument(name = "GET /api/v1/users", skip(state, current_user))]
async fn get_users(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            let users = UserRead::get_all(&state.db_conn).await?;
            Ok((StatusCode::OK, Json(users).into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    }
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
