use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::group::GroupType;
use koudaisai_portal_backend::domain::group_id::GroupId;
use koudaisai_portal_backend::domain::membership::Membership;
use koudaisai_portal_backend::domain::target_specifier::TargetSpecifier;
use koudaisai_portal_backend::domain::user_id::UserId;
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Deserialize)]
struct FromStrCase {
    input: String,
    ok: bool,
    expected: Option<String>,
}

pub fn test_from_str(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: FromStrCase = serde_json::from_str(&contents)?;
    let result = TargetSpecifier::from_str(&c.input);
    if c.ok {
        let ts = result.expect("expected Ok");
        if let Some(expected_s) = c.expected {
            let actual_s: String = (&ts).into();
            assert_eq!(actual_s, expected_s);
        }
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.input);
    }
    Ok(())
}

#[derive(Deserialize)]
struct DoesActorMatchCase {
    target: String,
    actor: ActorSpec,
    expected: bool,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum ActorSpec {
    User {
        #[serde(default)]
        group_type: String,
        #[serde(default)]
        group_ids: Vec<String>,
        #[serde(default)]
        user_id: Option<String>,
    },
    Admin {
        #[serde(default)]
        user_id: Option<String>,
    },
    NoLogin,
}

fn uid(s: Option<&String>) -> UserId {
    s.map(|s| UserId::new(Uuid::parse_str(s).unwrap()))
        .unwrap_or_else(|| UserId::new(Uuid::new_v4()))
}

fn parse_group_type(s: &str) -> GroupType {
    let u = || UserId::new(Uuid::new_v4());
    match s {
        "press" => GroupType::Press {
            representative: u(),
        },
        "general" => GroupType::GeneralProject {
            representative1: u(),
            representative2: u(),
            representative3: u(),
        },
        "booth" => GroupType::BoothProject {
            representative1: u(),
            representative2: u(),
            representative3: u(),
        },
        "stage" => GroupType::StageProject {
            representative1: u(),
            representative2: u(),
            representative3: u(),
        },
        "labo" => GroupType::LabProject {
            representative: u(),
            operator: u(),
        },
        _ => GroupType::Press {
            representative: u(),
        },
    }
}

pub fn test_does_actor_match(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: DoesActorMatchCase = serde_json::from_str(&contents)?;
    let target = TargetSpecifier::from_str(&c.target).expect("invalid target in fixture");

    let actor = match &c.actor {
        ActorSpec::User {
            group_type,
            group_ids,
            user_id,
        } => {
            let user_id = uid(user_id.as_ref());
            let memberships = group_ids
                .iter()
                .map(|g| Membership::new(GroupId::from_str(g).unwrap(), user_id, &FixedClock))
                .collect();
            ActorContext::User {
                user_id,
                memberships,
                group_type: parse_group_type(group_type),
            }
        }
        ActorSpec::Admin { user_id } => ActorContext::Admin {
            user_id: uid(user_id.as_ref()),
            claims: vec![],
        },
        ActorSpec::NoLogin => ActorContext::NoLogin,
    };

    assert_eq!(
        target.does_actor_match(&actor),
        c.expected,
        "failed for target: {}, actor: {:?}",
        c.target,
        c.actor
    );
    Ok(())
}
