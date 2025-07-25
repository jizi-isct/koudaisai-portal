use crate::entities::approval_request::{CreateApprovalRequest, ReadApprovalRequest};
use crate::entities::user::UserRead;
use crate::entities::user_id::UserId;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::StatusCode;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "init /api/v2/users/{user_id}/approval-requests")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_approval_requests).post(post_approval_request))
        .route("/{request_id}", get(get_approval_request))
        .route("/{request_id}/close", get(close_approval_request))
}

#[instrument(name = "GET /api/v2/users/{user_id}/approval-requests", skip(state))]
async fn get_approval_requests(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<UserId>,
) -> AppResponse {
    match current_user {
        CurrentUser::User(claims) => {
            let current_user = UserRead::from_claims(claims, &state.db_conn).await?;
            let user =
                UserRead::from_user_id(user_id, current_user.clone(), &state.db_conn).await?;
            if user.group_id != current_user.group_id {
                return Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()));
            }
            let requests = user.get_approval_requests(&state.db_conn).await?;
            Ok((StatusCode::OK, Json(requests).into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
    }
}

#[instrument(name = "POST /api/v2/users/{user_id}/approval-requests", skip(state))]
async fn post_approval_request(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<UserId>,
    Json(approval_request): Json<CreateApprovalRequest>,
) -> AppResponse {
    match current_user {
        CurrentUser::User(claims) => {
            let current_user = UserRead::from_claims(claims, &state.db_conn).await?;
            if !matches!(user_id, UserId::Me) {
                return Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()));
            }
            approval_request
                .insert(&state.db_conn, current_user.id)
                .await?;
            Ok((StatusCode::CREATED, ().into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
    }
}

#[instrument(
    name = "GET /api/v2/users/{user_id}/approval-requests/{request_id}",
    skip(state)
)]
async fn get_approval_request(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<UserId>,
    Path(request_id): Path<Uuid>,
) -> AppResponse {
    match current_user {
        CurrentUser::User(claims) => {
            let current_user = UserRead::from_claims(claims, &state.db_conn).await?;
            let user =
                UserRead::from_user_id(user_id, current_user.clone(), &state.db_conn).await?;
            let request = ReadApprovalRequest::find_from_id(request_id, &state.db_conn).await?;
            if user.group_id == current_user.group_id {
                Ok((StatusCode::OK, Json(request).into_response()))
            } else {
                Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()))
            }
        }
        _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
    }
}

#[instrument(
    name = "POST /api/v2/users/{user_id}/approval-requests/{request_id}/close",
    skip(state)
)]
async fn close_approval_request(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(user_id): Path<UserId>,
    Path(request_id): Path<Uuid>,
) -> AppResponse {
    match current_user {
        CurrentUser::User(claims) => {
            let current_user = UserRead::from_claims(claims, &state.db_conn).await?;
            let user =
                UserRead::from_user_id(user_id, current_user.clone(), &state.db_conn).await?;
            let request = ReadApprovalRequest::find_from_id(request_id, &state.db_conn).await?;
            let request = match request {
                Some(req) => req,
                None => return Ok((StatusCode::NOT_FOUND, "Not found.".into_response())),
            };
            if user.group_id == current_user.group_id {
                request.close(&state.db_conn).await?;
                Ok((StatusCode::NO_CONTENT, ().into_response()))
            } else {
                Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()))
            }
        }
        _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
    }
}
