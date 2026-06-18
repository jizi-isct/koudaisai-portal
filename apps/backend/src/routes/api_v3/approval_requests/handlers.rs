use super::dto::{ApprovalActionBody, ApprovalRequestCreate, ApprovalRequestRead};
use axum::Json;
use axum::extract::{Path, Query};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct ApprovalRequestPath {
    id: Uuid,
}

#[derive(Deserialize, IntoParams)]
pub struct ApprovalRequestQuery {
    group_id: Option<String>,
}

#[http_response]
pub enum GetApprovalRequestsResponse {
    #[response(status = OK)]
    Ok(Vec<ApprovalRequestRead>),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
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
    Query(query): Query<ApprovalRequestQuery>,
) -> GetApprovalRequestsResponse {
    todo!()
}

#[http_response]
pub enum PostApprovalRequestResponse {
    #[response(status = CREATED)]
    Created(ApprovalRequestRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid approval request")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
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
    Json(body): Json<ApprovalRequestCreate>,
) -> PostApprovalRequestResponse {
    todo!()
}

#[http_response]
pub enum GetApprovalRequestResponse {
    #[response(status = OK, description = "Approval request found")]
    Ok(ApprovalRequestRead),
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
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
    Path(path): Path<ApprovalRequestPath>,
) -> GetApprovalRequestResponse {
    todo!()
}

#[http_response]
pub enum ApproveApprovalRequestResponse {
    #[response(status = OK, description = "Approval request approved")]
    Ok(ApprovalRequestRead),
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = CONFLICT, description = "Approval request is not pending")]
    Conflict,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
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
    Path(path): Path<ApprovalRequestPath>,
    Json(body): Json<ApprovalActionBody>,
) -> ApproveApprovalRequestResponse {
    todo!()
}

#[http_response]
pub enum RejectApprovalRequestResponse {
    #[response(status = OK, description = "Approval request rejected")]
    Ok(ApprovalRequestRead),
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = CONFLICT, description = "Approval request is not pending")]
    Conflict,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
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
    Path(path): Path<ApprovalRequestPath>,
    Json(body): Json<ApprovalActionBody>,
) -> RejectApprovalRequestResponse {
    todo!()
}

#[http_response]
pub enum CloseApprovalRequestResponse {
    #[response(status = OK, description = "Approval request closed")]
    Ok(ApprovalRequestRead),
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = CONFLICT, description = "Approval request is not pending")]
    Conflict,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
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
    Path(path): Path<ApprovalRequestPath>,
) -> CloseApprovalRequestResponse {
    todo!()
}

#[http_response]
pub enum DeleteApprovalRequestResponse {
    #[response(status = NO_CONTENT, description = "Approval request deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Approval request not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
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
    Path(path): Path<ApprovalRequestPath>,
) -> DeleteApprovalRequestResponse {
    todo!()
}
