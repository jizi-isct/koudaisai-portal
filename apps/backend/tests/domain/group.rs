use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::{
    group::{Group, GroupType},
    group_id::GroupId,
};
use serde::Deserialize;
use std::path::Path;

fn make_group(group_type: GroupType) -> Group {
    Group::register(
        GroupId::new('A', 1).unwrap(),
        "Test Group".to_string(),
        group_type,
        &FixedClock,
    )
    .unwrap()
}

// --- register ---

#[derive(Deserialize)]
struct RegisterCase {
    name: String,
    ok: bool,
}

pub fn test_register(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: RegisterCase = serde_json::from_str(&contents)?;
    let result = Group::register(
        GroupId::new('A', 1).unwrap(),
        c.name.clone(),
        GroupType::Press,
        &FixedClock,
    );
    if c.ok {
        let group = result.unwrap_or_else(|_| panic!("expected Ok for {:?}", c.name));
        assert_eq!(group.name(), c.name.as_str());
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
    let mut group = make_group(GroupType::Press);
    let result = group.rename(c.new_name.clone(), &FixedClock);
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_name);
        assert_eq!(group.name(), c.new_name.as_str());
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_name);
    }
    Ok(())
}
