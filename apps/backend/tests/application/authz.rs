use crate::application::common::{
    ActorSpec, build_actor, mem, parse_group_type, parse_target, uid,
};
use crate::domain::common::FixedClock;
use chrono::Utc;
use koudaisai_portal_backend::application::authz::*;
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::approval_request::{
    ApprovalRequest, ApprovalRequestStatus, ApprovalRequestType,
};
use koudaisai_portal_backend::domain::approval_request_id::ApprovalRequestId;
use koudaisai_portal_backend::domain::form::{Form, FormType};
use koudaisai_portal_backend::domain::form_id::FormId;
use koudaisai_portal_backend::domain::group_id::GroupId;
use koudaisai_portal_backend::domain::membership::Membership;
use koudaisai_portal_backend::domain::target_specifier::TargetSpecifier;
use koudaisai_portal_backend::domain::user_id::UserId;
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

fn matches_result(result: &Result<(), CanGetByIdError>, expected: &str) -> bool {
    matches!(
        (result, expected),
        (Ok(()), "ok")
            | (Err(CanGetByIdError::Unauthorized), "unauthorized")
            | (Err(CanGetByIdError::NotFound), "not_found")
    )
}

fn make_request(issued_by: UserId) -> ApprovalRequest {
    ApprovalRequest::restore(
        ApprovalRequestId::generate(),
        Utc::now(),
        issued_by,
        GroupId::new('I', 1).unwrap(),
        ApprovalRequestType::EditExhibitionInfo {
            description: None,
            icon_key: None,
        },
        ApprovalRequestStatus::Pending,
        "reason".to_string(),
    )
}

// ---------------------------------------------------------------------------
// Case types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct BoolCase {
    actor: ActorSpec,
    expected: bool,
}

#[derive(Deserialize)]
struct ResultCase {
    actor: ActorSpec,
    expected: String,
}

// ---------------------------------------------------------------------------
// user
// ---------------------------------------------------------------------------

pub fn test_can_get_all_users(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_get_all_users(&ctx), c.expected);
    Ok(())
}

#[derive(Deserialize)]
struct GetUserByIdCase {
    actor: ActorSpec,
    #[serde(default)]
    target_group_ids: Vec<String>,
    expected: String,
}

pub fn test_can_get_user_by_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: GetUserByIdCase = serde_json::from_str(&contents)?;
    let target_uid = uid();
    let memberships_of_user: Vec<Membership> = c
        .target_group_ids
        .iter()
        .map(|g| mem(GroupId::from_str(g).unwrap(), target_uid))
        .collect();
    let (_, ctx) = build_actor(c.actor);
    let result = can_get_user_by_id(&ctx, memberships_of_user);
    assert!(
        matches_result(&result, &c.expected),
        "expected={}",
        c.expected
    );
    Ok(())
}

pub fn test_can_update_user(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    use koudaisai_portal_backend::domain::email_address::EmailAddress;
    use koudaisai_portal_backend::domain::user::User;
    let c: BoolCase = serde_json::from_str(&contents)?;
    let user = User::register(
        uid(),
        "test".to_string(),
        EmailAddress::new("test@example.com".to_string()).unwrap(),
        &FixedClock,
    )
    .unwrap();
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_update_user(&ctx, &user), c.expected);
    Ok(())
}

// ---------------------------------------------------------------------------
// settings
// ---------------------------------------------------------------------------

pub fn test_can_get_all_settings(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_get_all_settings(&ctx), c.expected);
    Ok(())
}

pub fn test_can_get_show_occasions_on_portal(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_get_show_occasions_on_portal(&ctx), c.expected);
    Ok(())
}

pub fn test_can_get_accept_correction_requests(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_get_accept_correction_requests(&ctx), c.expected);
    Ok(())
}

pub fn test_can_write_settings(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_write_settings(&ctx), c.expected);
    Ok(())
}

pub fn test_can_change_m_address_of_the_user(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_change_m_address_of_the_user(&ctx, uid()), c.expected);
    Ok(())
}

// ---------------------------------------------------------------------------
// group
// ---------------------------------------------------------------------------

pub fn test_can_get_all_groups(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_get_all_groups(&ctx), c.expected);
    Ok(())
}

#[derive(Deserialize)]
struct GetGroupByIdCase {
    actor: ActorSpec,
    group_id: String,
    expected: String,
}

pub fn test_can_get_group_by_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: GetGroupByIdCase = serde_json::from_str(&contents)?;
    let group_id = GroupId::from_str(&c.group_id).unwrap();
    let (actor_uid, ctx) = build_actor(c.actor);
    // Build `members` from the actor's memberships that match the queried group_id.
    // This means: if the actor listed group_id in their group_ids, they are in members.
    let members: Vec<Membership> = match &ctx {
        ActorContext::User { memberships, .. } => memberships
            .iter()
            .filter(|m| m.group_id() == group_id)
            .map(|_| mem(group_id, actor_uid))
            .collect(),
        _ => vec![],
    };
    let result = can_get_group_by_id(&ctx, &members);
    assert!(
        matches_result(&result, &c.expected),
        "expected={}",
        c.expected
    );
    Ok(())
}

pub fn test_can_create_group(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_create_group(&ctx), c.expected);
    Ok(())
}

// ---------------------------------------------------------------------------
// form
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct GetFormCase {
    actor: ActorSpec,
    form_targets: Vec<String>,
    expected: bool,
}

pub fn test_can_get_form(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: GetFormCase = serde_json::from_str(&contents)?;
    let now = Utc::now();
    let targets: Vec<TargetSpecifier> = c.form_targets.iter().map(|t| parse_target(t)).collect();
    let form = Form::restore(
        FormId::new(Uuid::new_v4()),
        now,
        now,
        Uuid::new_v4(),
        Uuid::new_v4(),
        targets,
        "form".to_string(),
        "summary".to_string(),
        now,
        FormType::TypeExternal {
            form_url: "https://example.com".to_string(),
        },
    );
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_get_form(&ctx, &form), c.expected);
    Ok(())
}

pub fn test_can_create_form(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_create_form(&ctx), c.expected);
    Ok(())
}

pub fn test_can_update_form(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_update_form(&ctx), c.expected);
    Ok(())
}

pub fn test_can_delete_form(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_delete_form(&ctx), c.expected);
    Ok(())
}

// ---------------------------------------------------------------------------
// approval_request
// ---------------------------------------------------------------------------

pub fn test_can_get_all_approval_requests(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_get_all_approval_requests(&ctx), c.expected);
    Ok(())
}

#[derive(Deserialize)]
struct GetGroupApprovalRequestsCase {
    actor: ActorSpec,
    group_id: String,
    expected: bool,
}

pub fn test_can_get_group_approval_requests(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: GetGroupApprovalRequestsCase = serde_json::from_str(&contents)?;
    let group_id = GroupId::from_str(&c.group_id).unwrap();
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_get_group_approval_requests(&ctx, group_id), c.expected);
    Ok(())
}

#[derive(Deserialize)]
struct GetApprovalRequestCase {
    actor: ActorSpec,
    #[serde(default)]
    issuer_group_ids: Vec<String>,
    expected: bool,
}

pub fn test_can_get_approval_request(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: GetApprovalRequestCase = serde_json::from_str(&contents)?;
    let issuer_uid = uid();
    let request = make_request(issuer_uid);
    let memberships_of_issuer: Vec<Membership> = c
        .issuer_group_ids
        .iter()
        .map(|g| mem(GroupId::from_str(g).unwrap(), issuer_uid))
        .collect();
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(
        can_get_approval_request(&ctx, &request, &memberships_of_issuer),
        c.expected
    );
    Ok(())
}

pub fn test_can_create_approval_request(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_create_approval_request(&ctx), c.expected);
    Ok(())
}

pub fn test_can_approve_or_reject_approval_request(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_approve_or_reject_approval_request(&ctx), c.expected);
    Ok(())
}

#[derive(Deserialize)]
struct CloseApprovalRequestCase {
    actor: ActorSpec,
    #[serde(default)]
    actor_is_issuer: bool,
    expected: bool,
}

pub fn test_can_close_approval_request(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: CloseApprovalRequestCase = serde_json::from_str(&contents)?;
    let issuer_uid = uid();
    let request = make_request(issuer_uid);
    let (_, ctx) = if c.actor_is_issuer {
        // Override the actor's user_id to match the request's issued_by.
        match c.actor {
            ActorSpec::User {
                group_type,
                group_ids,
            } => {
                let memberships: Vec<Membership> = group_ids
                    .iter()
                    .map(|g| mem(GroupId::from_str(g).unwrap(), issuer_uid))
                    .collect();
                let gt = parse_group_type(&group_type);
                (
                    issuer_uid,
                    ActorContext::User {
                        name: "テストユーザー".to_string(),
                        user_id: issuer_uid,
                        memberships,
                        group_type: gt,
                    },
                )
            }
            spec => build_actor(spec),
        }
    } else {
        build_actor(c.actor)
    };
    assert_eq!(can_close_approval_request(&ctx, &request), c.expected);
    Ok(())
}

pub fn test_can_delete_approval_request(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_delete_approval_request(&ctx), c.expected);
    Ok(())
}

// ---------------------------------------------------------------------------
// document_category
// ---------------------------------------------------------------------------

pub fn test_can_get_document_category_by_id(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: ResultCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    let result = can_get_document_category_by_id(&ctx);
    assert!(
        matches_result(&result, &c.expected),
        "expected={}",
        c.expected
    );
    Ok(())
}

pub fn test_can_get_all_document_categories(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_get_all_document_categories(&ctx), c.expected);
    Ok(())
}

pub fn test_can_create_document_category(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_create_document_category(&ctx), c.expected);
    Ok(())
}

pub fn test_can_update_document_category(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_update_document_category(&ctx), c.expected);
    Ok(())
}

pub fn test_can_delete_document_category(
    _path: &Path,
    contents: String,
) -> datatest_stable::Result<()> {
    let c: BoolCase = serde_json::from_str(&contents)?;
    let (_, ctx) = build_actor(c.actor);
    assert_eq!(can_delete_document_category(&ctx), c.expected);
    Ok(())
}
