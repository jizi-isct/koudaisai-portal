use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::{
    actor_ctx::ActorContext,
    group::GroupType,
    group_id::GroupId,
    membership::Membership,
    user_id::UserId,
};
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

fn user_ctx(group_type: GroupType) -> (UserId, ActorContext) {
    let user_id = UserId::new(Uuid::new_v4());
    let group_id = GroupId::new('G', 1).unwrap();
    let memberships = vec![Membership::new(group_id, user_id, &FixedClock)];
    (user_id, ActorContext::User { user_id, memberships, group_type })
}

fn parse_actor(s: &str) -> ActorContext {
    let u = || UserId::new(Uuid::new_v4());
    match s {
        "user_general" => user_ctx(GroupType::GeneralProject { representative1: u(), representative2: u(), representative3: u() }).1,
        "user_booth"   => user_ctx(GroupType::BoothProject   { representative1: u(), representative2: u(), representative3: u() }).1,
        "user_stage"   => user_ctx(GroupType::StageProject   { representative1: u(), representative2: u(), representative3: u() }).1,
        "user_labo"    => user_ctx(GroupType::LabProject     { representative: u() }).1,
        "user_press"   => user_ctx(GroupType::Press          { representative: u() }).1,
        "user"         => user_ctx(GroupType::Press { representative: u() }).1,
        "admin"        => ActorContext::Admin { user_id: u(), claims: vec![] },
        "nologin"      => ActorContext::NoLogin,
        s => panic!("unknown actor: {s}"),
    }
}

// --- is_group_type ---

#[derive(Deserialize)]
struct IsGroupTypeCase {
    actor: String,
    query: String,
    expected: bool,
}

pub fn test_is_group_type(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: IsGroupTypeCase = serde_json::from_str(&contents)?;
    let ctx = parse_actor(&c.actor);
    let result = match c.query.as_str() {
        "project_general" => ctx.is_group_type_project_general(),
        "project_booth"   => ctx.is_group_type_project_booth(),
        "project_stage"   => ctx.is_group_type_project_stage(),
        "project_labo"    => ctx.is_group_type_project_labo(),
        "press"           => ctx.is_group_type_press(),
        s => panic!("unknown query: {s}"),
    };
    assert_eq!(result, c.expected, "actor={:?} query={:?}", c.actor, c.query);
    Ok(())
}

// --- is_group_id ---

#[derive(Deserialize)]
struct IsGroupIdCase {
    actor: String,
    actor_group_id: Option<String>,
    query_group_id: String,
    expected: bool,
}

pub fn test_is_group_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: IsGroupIdCase = serde_json::from_str(&contents)?;
    let ctx = match c.actor.as_str() {
        "user" => {
            let user_id = UserId::new(Uuid::new_v4());
            let group_id = GroupId::from_str(c.actor_group_id.as_deref().unwrap()).unwrap();
            let memberships = vec![Membership::new(group_id, user_id, &FixedClock)];
            ActorContext::User {
                user_id,
                memberships,
                group_type: GroupType::Press { representative: user_id },
            }
        }
        "nologin" => ActorContext::NoLogin,
        s => panic!("unknown actor: {s}"),
    };
    let query = GroupId::from_str(&c.query_group_id).unwrap();
    println!("actor={:?} query={:?}", c.actor, c.query_group_id);
    assert_eq!(ctx.is_group_id(&query), c.expected, "actor={:?} query={:?}", c.actor, c.query_group_id);
    Ok(())
}

// --- is_user_id ---

#[derive(Deserialize)]
struct IsUserIdCase {
    actor: String,
    is_same_user: bool,
    expected: bool,
}

pub fn test_is_user_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: IsUserIdCase = serde_json::from_str(&contents)?;
    let actor_user_id = UserId::new(Uuid::new_v4());
    let query_user_id = if c.is_same_user { actor_user_id } else { UserId::new(Uuid::new_v4()) };

    let ctx = match c.actor.as_str() {
        "user"    => ActorContext::User {
            user_id: actor_user_id,
            memberships: vec![],
            group_type: GroupType::Press { representative: actor_user_id },
        },
        "admin"   => ActorContext::Admin { user_id: actor_user_id, claims: vec![] },
        "nologin" => ActorContext::NoLogin,
        s => panic!("unknown actor: {s}"),
    };
    assert_eq!(ctx.is_user_id(&query_user_id), c.expected, "actor={:?} same={}", c.actor, c.is_same_user);
    Ok(())
}

// --- is_user_nologin ---

#[derive(Deserialize)]
struct IsUserNologinCase {
    actor: String,
    expected: bool,
}

pub fn test_is_user_nologin(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: IsUserNologinCase = serde_json::from_str(&contents)?;
    let ctx = parse_actor(&c.actor);
    assert_eq!(ctx.is_user_nologin(), c.expected, "actor={:?}", c.actor);
    Ok(())
}