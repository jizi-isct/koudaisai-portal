// TODO(sqlx移行): 旧 entities 層は削除済み。型は application 層へ再配線する。
// use crate::entities::approval_request::{CreateApprovalRequest, ReadApprovalRequest};
// use crate::entities::user::UserRead;
// use crate::entities::user_id::UserId;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::Host;
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
        .route("/{request_id}/close", post(close_approval_request))
}

// TODO(sqlx移行): シグネチャが削除済みの entities::user_id::UserId を参照しているため最小スタブに差し替え。
// 元のシグネチャ・本体:
// #[instrument(name = "GET /api/v2/users/{user_id}/approval-requests", skip(state))]
// async fn get_approval_requests(
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Path(user_id): Path<UserId>,
// ) -> AppResponse {
//     match current_user {
//         CurrentUser::User(claims) => {
//             // TODO(sqlx移行): application層へ再配線
//             let current_user = UserRead::from_claims(claims, &state.db_conn).await?;
//             let user =
//                 UserRead::from_user_id(user_id, current_user.clone(), &state.db_conn).await?;
//             if user.group_id != current_user.group_id {
//                 return Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()));
//             }
//             let requests = user.get_approval_requests(&state.db_conn).await?;
//             Ok((StatusCode::OK, Json(requests).into_response()))
//         }
//         _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
//     }
// }
async fn get_approval_requests() -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_IMPLEMENTED
}

// TODO(sqlx移行): シグネチャが削除済みの entities::user_id::UserId / entities::approval_request::CreateApprovalRequest を参照しているため最小スタブに差し替え。
// 元のシグネチャ・本体:
// #[instrument(name = "POST /api/v2/users/{user_id}/approval-requests", skip(state))]
// async fn post_approval_request(
//     Host(host): Host,
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Path(user_id): Path<UserId>,
//     Json(approval_request): Json<CreateApprovalRequest>,
// ) -> AppResponse {
//     match current_user {
//         CurrentUser::User(claims) => {
//             // TODO(sqlx移行): application層へ再配線
//             let current_user = UserRead::from_claims(claims, &state.db_conn).await?;
//             if !matches!(user_id, UserId::Me) {
//                 return Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()));
//             }
//             let approval_request_id = approval_request
//                 .clone()
//                 .insert(&state.db_conn, current_user.id)
//                 .await?;
//             state
//                 .discord
//                 .send_approval_request_issue_message(
//                     &*format!("https://{}", host),
//                     &approval_request_id,
//                     &approval_request,
//                     &current_user,
//                     &state.s3_client,
//                     &state.s3_bucket,
//                 )
//                 .await?;
//             Ok((StatusCode::CREATED, ().into_response()))
//         }
//         _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
//     }
// }
async fn post_approval_request() -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_IMPLEMENTED
}

// TODO(sqlx移行): シグネチャが削除済みの entities::user_id::UserId を参照しているため最小スタブに差し替え。
// 元のシグネチャ・本体:
// #[instrument(
//     name = "GET /api/v2/users/{user_id}/approval-requests/{request_id}",
//     skip(state)
// )]
// async fn get_approval_request(
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Path((user_id, request_id)): Path<(UserId, Uuid)>,
// ) -> AppResponse {
//     match current_user {
//         CurrentUser::User(claims) => {
//             // TODO(sqlx移行): application層へ再配線
//             let current_user = UserRead::from_claims(claims, &state.db_conn).await?;
//             let user =
//                 UserRead::from_user_id(user_id, current_user.clone(), &state.db_conn).await?;
//             if user.group_id != current_user.group_id {
//                 return Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()));
//             }
//             let group = user.get_group_read(&state.db_conn).await?;
//             let request =
//                 match ReadApprovalRequest::find_from_id(request_id, &state.db_conn).await? {
//                     Some(req) => req,
//                     None => return Ok((StatusCode::NOT_FOUND, "Not found.".into_response())),
//                 };
//             if group.contains_user_in_representatives(request.issued_by) {
//                 Ok((StatusCode::OK, Json(request).into_response()))
//             } else {
//                 Ok((StatusCode::NOT_FOUND, "Not Found.".into_response()))
//             }
//         }
//         _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
//     }
// }
async fn get_approval_request() -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_IMPLEMENTED
}

// TODO(sqlx移行): シグネチャが削除済みの entities::user_id::UserId を参照しているため最小スタブに差し替え。
// 元のシグネチャ・本体:
// #[instrument(
//     name = "POST /api/v2/users/{user_id}/approval-requests/{request_id}/close",
//     skip(state)
// )]
// async fn close_approval_request(
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Path((user_id, request_id)): Path<(UserId, Uuid)>,
// ) -> AppResponse {
//     match current_user {
//         CurrentUser::User(claims) => {
//             // TODO(sqlx移行): application層へ再配線
//             let current_user = UserRead::from_claims(claims, &state.db_conn).await?;
//             let user =
//                 UserRead::from_user_id(user_id, current_user.clone(), &state.db_conn).await?;
//             if user.group_id != current_user.group_id {
//                 return Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response()));
//             }
//             let group = user.get_group_read(&state.db_conn).await?;
//             let request = ReadApprovalRequest::find_from_id(request_id, &state.db_conn).await?;
//             let request = match request {
//                 Some(req) => req,
//                 None => return Ok((StatusCode::NOT_FOUND, "Not found.".into_response())),
//             };
//             if group.contains_user_in_representatives(request.issued_by) {
//                 request.close(&state.db_conn).await?;
//                 Ok((StatusCode::NO_CONTENT, ().into_response()))
//             } else {
//                 Ok((StatusCode::NOT_FOUND, "Not Found.".into_response()))
//             }
//         }
//         _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
//     }
// }
async fn close_approval_request() -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_IMPLEMENTED
}
