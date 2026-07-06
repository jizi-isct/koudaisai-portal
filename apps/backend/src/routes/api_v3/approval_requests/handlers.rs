use super::super::ErrorMessage;
use super::super::V3State;
use super::dto::{ApprovalActionBody, ApprovalRequestCreate, ApprovalRequestRead};
use crate::application::error::{ApplicationOperationError, DeleteError, UpdateError};
use crate::domain::actor_ctx::ActorContext;
use crate::domain::approval_request_id::ApprovalRequestId;
use axum::Json;
use axum::extract::{Path, Query, State};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct ApprovalRequestPath {
    id: Uuid,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ApprovalRequestQuery {
    /// グループ絞り込み(現状はクエリ受理のみ。範囲は呼び出し元の所属グループに限定)。
    #[allow(dead_code)]
    group_id: Option<String>,
}

#[http_response]
pub enum GetApprovalRequestsResponse {
    #[response(status = OK)]
    Ok(Vec<ApprovalRequestRead>),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    get,
    description = "Get all approval requests, optionally filtered by group.",
    params(ApprovalRequestQuery),
    path = "/",
    responses(GetApprovalRequestsResponse),
    tag = super::super::APPROVAL_REQUESTS_TAG
)]
pub async fn get_approval_requests(
    State(st): State<V3State>,
    actor: ActorContext,
    Query(_query): Query<ApprovalRequestQuery>,
) -> GetApprovalRequestsResponse {
    // app に group_id 指定の use-case が無いため、group_id クエリは受理するが
    // グループ範囲は呼び出し元の所属に限定する。
    let result = if matches!(actor, crate::domain::actor_ctx::ActorContext::Admin { .. }) {
        st.app.approval_request().get_all(&actor).await
    } else if let Some(uid) = actor.user_id() {
        st.app
            .approval_request()
            .get_by_group_members(&actor, uid)
            .await
    } else {
        return GetApprovalRequestsResponse::Forbidden(ErrorMessage::forbidden());
    };
    match result {
        Ok(v) => GetApprovalRequestsResponse::Ok(v.iter().map(ApprovalRequestRead::from).collect()),
        Err(ApplicationOperationError::Unauthorized) => {
            GetApprovalRequestsResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(_) => {
            GetApprovalRequestsResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}

#[http_response]
pub enum PostApprovalRequestResponse {
    #[response(status = CREATED)]
    Created(ApprovalRequestRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid approval request")]
    UnprocessableEntity(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    post,
    description = "Create an approval request.",
    path = "/",
    responses(PostApprovalRequestResponse),
    request_body = ApprovalRequestCreate,
    tag = super::super::APPROVAL_REQUESTS_TAG
)]
pub async fn post_approval_request(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<ApprovalRequestCreate>,
) -> PostApprovalRequestResponse {
    let r#type = (&body.r#type).into();
    match st
        .app
        .approval_request()
        .create(&actor, r#type, body.issue_reason)
        .await
    {
        Ok(id) => match st.app.approval_request().get_by_id(&actor, id).await {
            Ok(Some(ar)) => PostApprovalRequestResponse::Created(ApprovalRequestRead::from(&ar)),
            _ => PostApprovalRequestResponse::InternalServerError(
                ErrorMessage::internal_server_error(),
            ),
        },
        Err(ApplicationOperationError::Unauthorized) => {
            PostApprovalRequestResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::InvalidInput(_)) => {
            PostApprovalRequestResponse::UnprocessableEntity(ErrorMessage::unprocessable_entity())
        }
        Err(_) => {
            PostApprovalRequestResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}

#[http_response]
pub enum GetApprovalRequestResponse {
    #[response(status = OK, description = "Approval request found")]
    Ok(ApprovalRequestRead),
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound(ErrorMessage),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    get,
    description = "Get an approval request by id.",
    params(ApprovalRequestPath),
    path = "/{id}",
    responses(GetApprovalRequestResponse),
    tag = super::super::APPROVAL_REQUESTS_TAG
)]
pub async fn get_approval_request(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<ApprovalRequestPath>,
) -> GetApprovalRequestResponse {
    match st
        .app
        .approval_request()
        .get_by_id(&actor, ApprovalRequestId::new(path.id))
        .await
    {
        Ok(Some(ar)) => GetApprovalRequestResponse::Ok(ApprovalRequestRead::from(&ar)),
        Ok(None) => GetApprovalRequestResponse::NotFound(ErrorMessage::not_found()),
        Err(ApplicationOperationError::Unauthorized) => {
            GetApprovalRequestResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(_) => {
            GetApprovalRequestResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}

#[http_response]
pub enum ApproveApprovalRequestResponse {
    #[response(status = OK, description = "Approval request approved")]
    Ok(ApprovalRequestRead),
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound(ErrorMessage),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = CONFLICT, description = "Approval request is not pending")]
    Conflict(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    post,
    description = "Approve an approval request by id.",
    params(ApprovalRequestPath),
    path = "/{id}/approve",
    responses(ApproveApprovalRequestResponse),
    request_body = ApprovalActionBody,
    tag = super::super::APPROVAL_REQUESTS_TAG
)]
pub async fn approve_approval_request(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<ApprovalRequestPath>,
    Json(body): Json<ApprovalActionBody>,
) -> ApproveApprovalRequestResponse {
    match st
        .app
        .approval_request()
        .approve(&actor, ApprovalRequestId::new(path.id), body.reason)
        .await
    {
        Ok(ar) => ApproveApprovalRequestResponse::Ok(ApprovalRequestRead::from(&ar)),
        Err(ApplicationOperationError::Unauthorized) => {
            ApproveApprovalRequestResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            ApproveApprovalRequestResponse::NotFound(ErrorMessage::not_found())
        }
        Err(ApplicationOperationError::InvalidInput(_)) => {
            ApproveApprovalRequestResponse::Conflict(ErrorMessage::conflict())
        }
        Err(_) => ApproveApprovalRequestResponse::InternalServerError(
            ErrorMessage::internal_server_error(),
        ),
    }
}

#[http_response]
pub enum RejectApprovalRequestResponse {
    #[response(status = OK, description = "Approval request rejected")]
    Ok(ApprovalRequestRead),
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound(ErrorMessage),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = CONFLICT, description = "Approval request is not pending")]
    Conflict(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    post,
    description = "Reject an approval request by id.",
    params(ApprovalRequestPath),
    path = "/{id}/reject",
    responses(RejectApprovalRequestResponse),
    request_body = ApprovalActionBody,
    tag = super::super::APPROVAL_REQUESTS_TAG
)]
pub async fn reject_approval_request(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<ApprovalRequestPath>,
    Json(body): Json<ApprovalActionBody>,
) -> RejectApprovalRequestResponse {
    match st
        .app
        .approval_request()
        .reject(&actor, ApprovalRequestId::new(path.id), body.reason)
        .await
    {
        Ok(ar) => RejectApprovalRequestResponse::Ok(ApprovalRequestRead::from(&ar)),
        Err(ApplicationOperationError::Unauthorized) => {
            RejectApprovalRequestResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            RejectApprovalRequestResponse::NotFound(ErrorMessage::not_found())
        }
        Err(ApplicationOperationError::InvalidInput(_)) => {
            RejectApprovalRequestResponse::Conflict(ErrorMessage::conflict())
        }
        Err(_) => {
            RejectApprovalRequestResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}

#[http_response]
pub enum CloseApprovalRequestResponse {
    #[response(status = OK, description = "Approval request closed")]
    Ok(ApprovalRequestRead),
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound(ErrorMessage),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = CONFLICT, description = "Approval request is not pending")]
    Conflict(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    post,
    description = "Close an approval request by id.",
    params(ApprovalRequestPath),
    path = "/{id}/close",
    responses(CloseApprovalRequestResponse),
    tag = super::super::APPROVAL_REQUESTS_TAG
)]
pub async fn close_approval_request(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<ApprovalRequestPath>,
) -> CloseApprovalRequestResponse {
    let id = ApprovalRequestId::new(path.id);
    match st.app.approval_request().close(&actor, id).await {
        Ok(()) => match st.app.approval_request().get_by_id(&actor, id).await {
            Ok(Some(ar)) => CloseApprovalRequestResponse::Ok(ApprovalRequestRead::from(&ar)),
            Ok(None) => CloseApprovalRequestResponse::NotFound(ErrorMessage::not_found()),
            _ => CloseApprovalRequestResponse::InternalServerError(
                ErrorMessage::internal_server_error(),
            ),
        },
        Err(ApplicationOperationError::Unauthorized) => {
            CloseApprovalRequestResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            CloseApprovalRequestResponse::NotFound(ErrorMessage::not_found())
        }
        Err(ApplicationOperationError::InvalidInput(_)) => {
            CloseApprovalRequestResponse::Conflict(ErrorMessage::conflict())
        }
        Err(_) => {
            CloseApprovalRequestResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}

#[http_response]
pub enum DeleteApprovalRequestResponse {
    #[response(status = NO_CONTENT, description = "Approval request deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound(ErrorMessage),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    delete,
    description = "Delete an approval request by id.",
    params(ApprovalRequestPath),
    path = "/{id}",
    responses(DeleteApprovalRequestResponse),
    tag = super::super::APPROVAL_REQUESTS_TAG
)]
pub async fn delete_approval_request(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<ApprovalRequestPath>,
) -> DeleteApprovalRequestResponse {
    match st
        .app
        .approval_request()
        .delete(&actor, ApprovalRequestId::new(path.id))
        .await
    {
        Ok(()) => DeleteApprovalRequestResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => {
            DeleteApprovalRequestResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteApprovalRequestResponse::NotFound(ErrorMessage::not_found())
        }
        Err(_) => {
            DeleteApprovalRequestResponse::InternalServerError(ErrorMessage::internal_server_error())
        }
    }
}
