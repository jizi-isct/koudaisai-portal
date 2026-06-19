use koudaisai_portal_backend::domain::{group::GroupType, membership::Membership};
use serde::Deserialize;
use std::path::Path;

// --- validate_set ---
//
// フィクスチャには Membership を直接（serde 表現で）記述する．
// 例:
// {
//   "group_type": "general_project",
//   "ok": true,
//   "memberships": [
//     { "group_id": "G-001", "user_id": "...uuid...", "role": "FirstResponsible", "created_at": "2026-01-01T00:00:00Z" }
//   ]
// }

#[derive(Deserialize)]
struct ValidateSetCase {
    group_type: String,
    memberships: Vec<Membership>,
    ok: bool,
}

fn parse_group_type(s: &str) -> GroupType {
    match s {
        "press" => GroupType::Press,
        "general_project" => GroupType::GeneralProject,
        "booth_project" => GroupType::BoothProject,
        "lab_project" => GroupType::LabProject,
        "stage_project" => GroupType::StageProject,
        s => panic!("unknown group_type: {s}"),
    }
}

pub fn test_validate_set(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: ValidateSetCase = serde_json::from_str(&contents)?;
    let group_type = parse_group_type(&c.group_type);
    let result = Membership::validate_set(group_type, &c.memberships);
    if c.ok {
        assert!(
            result.is_ok(),
            "expected Ok for {:?}, got {:?}",
            c.group_type,
            result
        );
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.group_type);
    }
    Ok(())
}
