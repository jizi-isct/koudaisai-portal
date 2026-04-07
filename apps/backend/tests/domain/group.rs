use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::{
    group::{Group, GroupType},
    group_id::GroupId,
    membership::Membership,
    user_id::UserId,
};
use serde::Deserialize;
use std::path::Path;
use uuid::Uuid;

fn make_group(group_type: GroupType) -> Group {
    Group::register(GroupId::new('A', 1).unwrap(), "Test Group".to_string(), group_type, &FixedClock)
        .unwrap()
}

fn parse_group_type(s: &str) -> GroupType {
    let u = || UserId::new(Uuid::new_v4());
    match s {
        "press"           => GroupType::Press { representative: u() },
        "general_project" => GroupType::GeneralProject { representative1: u(), representative2: u(), representative3: u() },
        "booth_project"   => GroupType::BoothProject  { representative1: u(), representative2: u(), representative3: u() },
        "lab_project"     => GroupType::LabProject    { representative: u() },
        "stage_project"   => GroupType::StageProject  { representative1: u(), representative2: u(), representative3: u() },
        s => panic!("unknown group_type: {s}"),
    }
}

// --- register ---

#[derive(Deserialize)]
struct RegisterCase {
    name: String,
    ok: bool,
}

pub fn test_register(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: RegisterCase = serde_json::from_str(&contents)?;
    let group_type = GroupType::Press { representative: UserId::new(Uuid::new_v4()) };
    let result = Group::register(GroupId::new('A', 1).unwrap(), c.name.clone(), group_type, &FixedClock);
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.name);
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.name);
    }
    Ok(())
}

// --- rename ---

#[derive(Deserialize)]
struct RenameCase {
    new_name: String,
    ok: bool,
}

pub fn test_rename(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: RenameCase = serde_json::from_str(&contents)?;
    let mut group = make_group(GroupType::Press { representative: UserId::new(Uuid::new_v4()) });
    let result = group.rename(c.new_name.clone(), &FixedClock);
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_name);
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_name);
    }
    Ok(())
}

// --- update_roles ---

#[derive(Deserialize)]
struct UpdateRolesCase {
    from_type: String,
    to_type: String,
    membership_matches: bool,
    ok: bool,
}

pub fn test_update_roles(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: UpdateRolesCase = serde_json::from_str(&contents)?;
    let mut group = make_group(parse_group_type(&c.from_type));
    let new_type = parse_group_type(&c.to_type);
    let membership_group_id = if c.membership_matches {
        group.id()
    } else {
        GroupId::new('Z', 99).unwrap()
    };
    let membership = Membership::new(membership_group_id, UserId::new(Uuid::new_v4()), &FixedClock);
    let result = group.update_roles(new_type, &FixedClock, &membership);
    if c.ok {
        assert!(result.is_ok(), "expected Ok: {:?}→{:?} match={}", c.from_type, c.to_type, c.membership_matches);
    } else {
        assert!(result.is_err(), "expected Err: {:?}→{:?} match={}", c.from_type, c.to_type, c.membership_matches);
    }
    Ok(())
}