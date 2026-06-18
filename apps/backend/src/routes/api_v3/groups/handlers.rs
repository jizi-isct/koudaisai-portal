use super::dto::{GroupCreate, GroupRead, GroupUpdate, MemberCreate, MemberRead, Role};
use axum::Json;
use axum::extract::Path;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;

#[http_response]
pub enum GetGroupsResponse {
    #[response(status = OK)]
    Ok(Vec<GroupRead>),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get all groups.",
    path = "/",
    responses(GetGroupsResponse),
    tag = super::super::GROUPS_TAG
)]
pub async fn get_groups() -> GetGroupsResponse {
    todo!()
}

#[derive(Deserialize, IntoParams)]
pub struct GroupPath {
    id: String,
}

#[http_response]
pub enum PutGroupResponse {
    #[response(status = CREATED)]
    Created(GroupRead),
    #[response(status = OK, description = "Group replaced")]
    Ok(GroupRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = CONFLICT, description = "Conflict")]
    Conflict,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    put,
    description = "Create a group by id or replace an existing one.",
    params(GroupPath),
    path = "/{id}",
    responses(PutGroupResponse),
    request_body=GroupCreate,
    tag = super::super::GROUPS_TAG
)]
pub async fn put_group(
    Path(path): Path<GroupPath>,
    Json(group): Json<GroupCreate>,
) -> PutGroupResponse {
    todo!()
}

#[http_response]
pub enum GetGroupResponse {
    #[response(status = OK, description = "Group found")]
    Ok(GroupRead),
    #[response(status = NOT_FOUND, description = "Group not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get a group by id.",
    params(GroupPath),
    path = "/{id}",
    responses(GetGroupResponse),
    tag = super::super::GROUPS_TAG
)]
pub async fn get_group(Path(path): Path<GroupPath>) -> GetGroupResponse {
    todo!()
}

#[http_response]
pub enum PatchGroupResponse {
    #[response(status = OK, description = "Group updated")]
    Ok(GroupRead),
    #[response(status = NOT_FOUND, description = "Group not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    patch,
    description = "Edit a group by id.",
    params(GroupPath),
    path="/{id}",
    responses(PatchGroupResponse),
    request_body=GroupUpdate,
    tag = super::super::GROUPS_TAG
)]
pub async fn patch_group(
    Path(path): Path<GroupPath>,
    Json(group): Json<GroupUpdate>,
) -> PatchGroupResponse {
    todo!()
}

#[http_response]
pub enum DeleteGroupResponse {
    #[response(status = NO_CONTENT, description = "Group deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Group not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Delete a group by id.",
    params(GroupPath),
    path="/{id}",
    responses(DeleteGroupResponse),
    tag = super::super::GROUPS_TAG
)]
pub async fn delete_group(Path(path): Path<GroupPath>) -> DeleteGroupResponse {
    todo!()
}

#[derive(Deserialize, IntoParams)]
pub struct MemberPath {
    id: String,
    role: Role,
}

#[http_response]
pub enum GetMembersResponse {
    #[response(status = OK)]
    Ok(Vec<MemberRead>),
    #[response(status = NOT_FOUND, description = "Group not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get all members of a group.",
    params(GroupPath),
    path = "/{id}/members",
    responses(GetMembersResponse),
    tag = super::super::GROUPS_TAG
)]
pub async fn get_members(Path(path): Path<GroupPath>) -> GetMembersResponse {
    todo!()
}

#[http_response]
pub enum PutMemberResponse {
    #[response(status = CREATED)]
    Created(MemberRead),
    #[response(status = OK, description = "Member replaced")]
    Ok(MemberRead),
    #[response(status = NOT_FOUND, description = "Group not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = UNPROCESSABLE_ENTITY, description = "Invalid for group type")]
    UnprocessableEntity,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    put,
    description = "Assign a member to a role in a group or replace an existing one.",
    params(MemberPath),
    path = "/{id}/members/{role}",
    responses(PutMemberResponse),
    request_body = MemberCreate,
    tag = super::super::GROUPS_TAG
)]
pub async fn put_member(
    Path(path): Path<MemberPath>,
    Json(body): Json<MemberCreate>,
) -> PutMemberResponse {
    todo!()
}

#[http_response]
pub enum DeleteMemberResponse {
    #[response(status = NO_CONTENT, description = "Member deleted")]
    NoContent,
    #[response(status = NOT_FOUND, description = "Group not found")]
    NotFound,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    delete,
    description = "Remove a member from a role in a group.",
    params(MemberPath),
    path = "/{id}/members/{role}",
    responses(DeleteMemberResponse),
    tag = super::super::GROUPS_TAG
)]
pub async fn delete_member(Path(path): Path<MemberPath>) -> DeleteMemberResponse {
    todo!()
}
