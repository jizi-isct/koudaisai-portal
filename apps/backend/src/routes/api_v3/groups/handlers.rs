use super::super::V3State;
use super::dto::{GroupCreate, GroupRead, GroupUpdate, MemberCreate, MemberRead, Role};
use crate::application::error::{
    ApplicationOperationError, ApplicationSequentialOperationError, DeleteError, InsertError,
    UpdateError,
};
use crate::domain::group_id::GroupId;
use crate::domain::membership::Role as DomainRole;
use crate::domain::user_id::UserId;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::domain::actor_ctx::ActorContext;
use axum::Json;
use axum::extract::{Path, State};
use serde::Deserialize;
use std::str::FromStr;
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
pub async fn get_groups(State(st): State<V3State>, actor: ActorContext) -> GetGroupsResponse {
    match st.app.group().get_all(&actor).await {
        Ok(groups) => GetGroupsResponse::Ok(groups.iter().map(GroupRead::from).collect()),
        Err(ApplicationOperationError::Unauthorized) => GetGroupsResponse::Forbidden,
        Err(_) => GetGroupsResponse::InternalServerError,
    }
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
    #[response(status = BAD_REQUEST, description = "Invalid group id or input")]
    BadRequest,
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
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
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<GroupPath>,
    Json(group): Json<GroupCreate>,
) -> PutGroupResponse {
    let Ok(group_id) = GroupId::from_str(&path.id) else {
        return PutGroupResponse::BadRequest;
    };
    let domain_type = (&group.r#type).into();
    let tx = SqliteTransaction::new(st.pool.clone());
    match st
        .app
        .group()
        .create_group(&actor, tx, group_id, group.name.clone(), domain_type)
        .await
    {
        Ok(()) => match st.app.group().get_by_id(&actor, group_id).await {
            Ok(Some(g)) => PutGroupResponse::Created(GroupRead::from(&g)),
            _ => PutGroupResponse::InternalServerError,
        },
        // 既存グループ → 名称を置換する(種別は変更不可)。
        Err(ApplicationSequentialOperationError::OperationFailed(InsertError::Conflict)) => {
            match st.app.group().rename_group(&actor, group_id, group.name).await {
                Ok(g) => PutGroupResponse::Ok(GroupRead::from(&g)),
                Err(ApplicationOperationError::Unauthorized) => PutGroupResponse::Forbidden,
                Err(ApplicationOperationError::InvalidInput(_)) => PutGroupResponse::BadRequest,
                Err(_) => PutGroupResponse::InternalServerError,
            }
        }
        Err(ApplicationSequentialOperationError::Unauthorized) => PutGroupResponse::Forbidden,
        Err(ApplicationSequentialOperationError::InvalidInput(_)) => PutGroupResponse::BadRequest,
        Err(_) => PutGroupResponse::InternalServerError,
    }
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
pub async fn get_group(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<GroupPath>,
) -> GetGroupResponse {
    let Ok(group_id) = GroupId::from_str(&path.id) else {
        return GetGroupResponse::NotFound;
    };
    match st.app.group().get_by_id(&actor, group_id).await {
        Ok(Some(g)) => GetGroupResponse::Ok(GroupRead::from(&g)),
        Ok(None) => GetGroupResponse::NotFound,
        Err(ApplicationOperationError::Unauthorized) => GetGroupResponse::Forbidden,
        Err(_) => GetGroupResponse::InternalServerError,
    }
}

#[http_response]
pub enum PatchGroupResponse {
    #[response(status = OK, description = "Group updated")]
    Ok(GroupRead),
    #[response(status = BAD_REQUEST, description = "Invalid input")]
    BadRequest,
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
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<GroupPath>,
    Json(group): Json<GroupUpdate>,
) -> PatchGroupResponse {
    let Ok(group_id) = GroupId::from_str(&path.id) else {
        return PatchGroupResponse::NotFound;
    };
    // name の指定が無ければ現状を取得して返す(no-op)。
    let Some(name) = group.name else {
        return match st.app.group().get_by_id(&actor, group_id).await {
            Ok(Some(g)) => PatchGroupResponse::Ok(GroupRead::from(&g)),
            Ok(None) => PatchGroupResponse::NotFound,
            Err(ApplicationOperationError::Unauthorized) => PatchGroupResponse::Forbidden,
            Err(_) => PatchGroupResponse::InternalServerError,
        };
    };
    match st.app.group().rename_group(&actor, group_id, name).await {
        Ok(g) => PatchGroupResponse::Ok(GroupRead::from(&g)),
        Err(ApplicationOperationError::Unauthorized) => PatchGroupResponse::Forbidden,
        Err(ApplicationOperationError::InvalidInput(_)) => PatchGroupResponse::BadRequest,
        Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound)) => {
            PatchGroupResponse::NotFound
        }
        Err(_) => PatchGroupResponse::InternalServerError,
    }
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
pub async fn delete_group(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<GroupPath>,
) -> DeleteGroupResponse {
    let Ok(group_id) = GroupId::from_str(&path.id) else {
        return DeleteGroupResponse::NotFound;
    };
    match st.app.group().delete_group(&actor, group_id).await {
        Ok(()) => DeleteGroupResponse::NoContent,
        Err(ApplicationOperationError::Unauthorized) => DeleteGroupResponse::Forbidden,
        Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteGroupResponse::NotFound
        }
        Err(_) => DeleteGroupResponse::InternalServerError,
    }
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
pub async fn get_members(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<GroupPath>,
) -> GetMembersResponse {
    let Ok(group_id) = GroupId::from_str(&path.id) else {
        return GetMembersResponse::NotFound;
    };
    match st.app.group().get_members(&actor, group_id).await {
        Ok(Some(members)) => GetMembersResponse::Ok(members.iter().map(MemberRead::from).collect()),
        Ok(None) => GetMembersResponse::NotFound,
        Err(ApplicationOperationError::Unauthorized) => GetMembersResponse::Forbidden,
        Err(_) => GetMembersResponse::InternalServerError,
    }
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
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<MemberPath>,
    Json(body): Json<MemberCreate>,
) -> PutMemberResponse {
    let Ok(group_id) = GroupId::from_str(&path.id) else {
        return PutMemberResponse::NotFound;
    };
    let role: DomainRole = (&path.role).into();
    let user_id = UserId::new(body.user_id);
    // PUT = 役職スロットの置換。空席化＋追加を単一トランザクションで原子的に行う
    // (途中失敗で空席のまま残さない)。置換なら 200、空席への新規なら 201。
    let tx = SqliteTransaction::new(st.pool.clone());
    match st
        .app
        .group()
        .replace_member(&actor, tx, group_id, user_id, role)
        .await
    {
        Ok(was_replace) => match st.app.group().get_members(&actor, group_id).await {
            Ok(Some(members)) => match members.iter().find(|m| m.role() == role) {
                Some(m) if was_replace => PutMemberResponse::Ok(MemberRead::from(m)),
                Some(m) => PutMemberResponse::Created(MemberRead::from(m)),
                None => PutMemberResponse::InternalServerError,
            },
            _ => PutMemberResponse::InternalServerError,
        },
        Err(ApplicationSequentialOperationError::Unauthorized) => PutMemberResponse::Forbidden,
        Err(ApplicationSequentialOperationError::InvalidInput(_)) => {
            PutMemberResponse::UnprocessableEntity
        }
        Err(_) => PutMemberResponse::InternalServerError,
    }
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
pub async fn delete_member(
    State(st): State<V3State>,
    actor: ActorContext,
    Path(path): Path<MemberPath>,
) -> DeleteMemberResponse {
    let Ok(group_id) = GroupId::from_str(&path.id) else {
        return DeleteMemberResponse::NotFound;
    };
    let role: DomainRole = (&path.role).into();
    let tx = SqliteTransaction::new(st.pool.clone());
    match st
        .app
        .group()
        .remove_member(&actor, tx, group_id, role)
        .await
    {
        Ok(()) => DeleteMemberResponse::NoContent,
        Err(ApplicationSequentialOperationError::Unauthorized) => DeleteMemberResponse::Forbidden,
        Err(ApplicationSequentialOperationError::OperationFailed(DeleteError::NotFound)) => {
            DeleteMemberResponse::NotFound
        }
        Err(_) => DeleteMemberResponse::InternalServerError,
    }
}
