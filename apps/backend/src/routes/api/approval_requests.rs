use crate::entities::approval_request::{delete_by_id, ReadApprovalRequest};
use crate::entities::notification::{NotificationCreate, NotificationType};
use crate::entities::target_specifier::TargetSpecifier;
use crate::entities::user::UserRead;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::Host;
use http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApproveRequest {
    pub approval_reason: Option<String>,
}

#[instrument(name = "init /api/v2/approval-requests")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_approval_requests))
        .route(
            "/{request_id}",
            get(get_approval_request).delete(delete_approval_request),
        )
        .route("/{request_id}/approve", post(approve_approval_request))
        .route("/{request_id}/reject", post(reject_approval_request))
}

#[instrument(name = "GET /api/v2/approval-requests", skip(state))]
async fn get_approval_requests(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            let response = ReadApprovalRequest::get_all(&state.db_conn).await?;
            Ok((StatusCode::OK, Json(response).into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
    }
}

#[instrument(name = "GET /api/v2/approval-requests/{request_id}", skip(state))]
async fn get_approval_request(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(request_id): Path<Uuid>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            let response = ReadApprovalRequest::find_from_id(request_id, &state.db_conn).await?;
            Ok((StatusCode::OK, Json(response).into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
    }
}

#[instrument(name = "DELETE /api/v2/approval-requests/{request_id}", skip(state))]
async fn delete_approval_request(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(request_id): Path<Uuid>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            delete_by_id(request_id, &state.db_conn).await?;
            Ok((StatusCode::NO_CONTENT, ().into_response()))
        }
        _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
    }
}

#[instrument(name = "POST /api/v2/approval-requests/approve", skip(state))]
async fn approve_approval_request(
    Host(host): Host,
    header_map: HeaderMap,
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(request_id): Path<Uuid>,
    Json(approve_request): Json<ApproveRequest>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(claims) => {
            let uuid = Uuid::from_str(claims.subject())?;
            let request = ReadApprovalRequest::find_from_id(request_id, &state.db_conn).await?;
            match request {
                Some(request) => {
                    let issuer_id = request.issued_by;
                    request
                        .approve(
                            Some(uuid),
                            approve_request.approval_reason,
                            state.clone(),
                            header_map
                                .get::<&str>("Authorization")
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .strip_prefix("Bearer ")
                                .unwrap(),
                        )
                        .await?;

                    // Send notification to issuer
                    let notification = NotificationCreate {
                        target: vec![TargetSpecifier::UserId(issuer_id)],
                        notification_type: NotificationType::TypeApprovalRequest {
                            approval_request_id: request_id,
                        },
                    };
                    notification.insert(&state.db_conn, Some(uuid)).await?;

                    // Send discord webhook
                    let approval_request =
                        ReadApprovalRequest::find_from_id(request_id, &state.db_conn)
                            .await?
                            .unwrap();
                    let issued_by =
                        UserRead::find_from_id(approval_request.issued_by, &state.db_conn)
                            .await?
                            .unwrap();
                    state
                        .discord
                        .send_approval_request_approval_message(
                            &*format!("https://{}", host),
                            &request_id,
                            &approval_request,
                            &issued_by,
                            &claims,
                        )
                        .await?;

                    Ok((StatusCode::NO_CONTENT, ().into_response()))
                }
                None => Ok((StatusCode::NOT_FOUND, "Request not found.".into_response())),
            }
        }
        _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
    }
}

#[instrument(name = "POST /api/v2/approval-requests/reject", skip(state))]
async fn reject_approval_request(
    Host(host): Host,
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(request_id): Path<Uuid>,
    Json(approve_request): Json<ApproveRequest>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(claims) => {
            let uuid = Uuid::from_str(claims.subject())?;
            let request = ReadApprovalRequest::find_from_id(request_id, &state.db_conn).await?;
            match request {
                Some(request) => {
                    let issuer_id = request.issued_by;
                    request
                        .reject(&state.db_conn, Some(uuid), approve_request.approval_reason)
                        .await?;

                    // Send notification to issuer
                    let notification = NotificationCreate {
                        target: vec![TargetSpecifier::UserId(issuer_id)],
                        notification_type: NotificationType::TypeApprovalRequest {
                            approval_request_id: request_id,
                        },
                    };
                    notification.insert(&state.db_conn, Some(uuid)).await?;

                    // Send discord webhook
                    let approval_request =
                        ReadApprovalRequest::find_from_id(request_id, &state.db_conn)
                            .await?
                            .unwrap();
                    let issued_by =
                        UserRead::find_from_id(approval_request.issued_by, &state.db_conn)
                            .await?
                            .unwrap();
                    state
                        .discord
                        .send_approval_request_approval_message(
                            &*format!("https://{}", host),
                            &request_id,
                            &approval_request,
                            &issued_by,
                            &claims,
                        )
                        .await?;

                    Ok((StatusCode::NO_CONTENT, ().into_response()))
                }
                None => Ok((StatusCode::NOT_FOUND, "Request not found.".into_response())),
            }
        }
        _ => Ok((StatusCode::FORBIDDEN, "Forbidden.".into_response())),
    }
}
