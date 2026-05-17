use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::group::GroupType;
use koudaisai_portal_backend::domain::group_id::GroupId;
use koudaisai_portal_backend::domain::membership::Membership;
use koudaisai_portal_backend::domain::target_specifier::TargetSpecifier;
use koudaisai_portal_backend::domain::user_id::UserId;
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

pub fn uid() -> UserId {
    UserId::new(Uuid::new_v4())
}

pub fn mem(group_id: GroupId, user_id: UserId) -> Membership {
    Membership::new(group_id, user_id, &FixedClock)
}

#[derive(Deserialize)]
pub enum ActorSpec {
    Admin {
        #[serde(default)]
        claims: Vec<String>,
    },
    User {
        #[serde(default)]
        group_type: String,
        #[serde(default)]
        group_ids: Vec<String>,
    },
    Nologin,
}

pub fn parse_group_type(s: &str) -> GroupType {
    match s {
        "" | "press" => GroupType::Press {
            representative: uid(),
        },
        "general" => GroupType::GeneralProject {
            representative1: uid(),
            representative2: uid(),
            representative3: uid(),
        },
        "booth" => GroupType::BoothProject {
            representative1: uid(),
            representative2: uid(),
            representative3: uid(),
        },
        "stage" => GroupType::StageProject {
            representative1: uid(),
            representative2: uid(),
            representative3: uid(),
        },
        "labo" => GroupType::LabProject {
            representative: uid(),
        },
        s => panic!("unknown group_type: {s}"),
    }
}

pub fn parse_target(s: &str) -> TargetSpecifier {
    match s {
        "press" => TargetSpecifier::GroupTypePress,
        "general" => TargetSpecifier::GroupTypeProjectGeneral,
        "booth" => TargetSpecifier::GroupTypeProjectBooth,
        "stage" => TargetSpecifier::GroupTypeProjectStage,
        "labo" => TargetSpecifier::GroupTypeProjectLabo,
        s => TargetSpecifier::from_str(s).unwrap(),
    }
}

pub fn build_actor(spec: ActorSpec) -> (UserId, ActorContext) {
    let user_id = uid();
    match spec {
        ActorSpec::Admin { claims } => (user_id, ActorContext::Admin { user_id, claims }),
        ActorSpec::User {
            group_type,
            group_ids,
        } => {
            let memberships: Vec<Membership> = group_ids
                .iter()
                .map(|g| mem(GroupId::from_str(g).unwrap(), user_id))
                .collect();
            let gt = parse_group_type(&group_type);
            (
                user_id,
                ActorContext::User {
                    user_id,
                    memberships,
                    group_type: gt,
                },
            )
        }
        ActorSpec::Nologin => (user_id, ActorContext::NoLogin),
    }
}

thread_local! {
    static RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
}

pub fn run<F: std::future::Future>(f: F) -> F::Output {
    RUNTIME.with(|rt| rt.block_on(f))
}
