use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::{
    group::GroupType, group_id::GroupId, membership::Membership, user_id::UserId,
};
use serde::Deserialize;
use std::path::Path;
use uuid::Uuid;

// --- from_group_type ---

#[derive(Deserialize)]
struct FromGroupTypeCase {
    group_type: String,
    expected_count: usize,
}

pub fn test_from_group_type(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: FromGroupTypeCase = serde_json::from_str(&contents)?;
    let group_id = GroupId::new('G', 1).unwrap();
    let u = || UserId::new(Uuid::new_v4());

    let group_type = match c.group_type.as_str() {
        "press" => GroupType::Press {
            representative: u(),
        },
        "general_project" => GroupType::GeneralProject {
            representative1: u(),
            representative2: u(),
            representative3: u(),
        },
        "booth_project" => GroupType::BoothProject {
            representative1: u(),
            representative2: u(),
            representative3: u(),
        },
        "lab_project" => GroupType::LabProject {
            representative: u(),
        },
        "stage_project" => GroupType::StageProject {
            representative1: u(),
            representative2: u(),
            representative3: u(),
        },
        s => panic!("unknown group_type: {s}"),
    };

    let memberships = Membership::from_group_type(group_id, &group_type, &FixedClock);
    assert_eq!(
        memberships.len(),
        c.expected_count,
        "membership count mismatch for {:?}",
        c.group_type
    );
    for m in &memberships {
        assert_eq!(m.group_id(), group_id);
    }
    Ok(())
}
