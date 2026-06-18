use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::email_address::EmailAddress;
use koudaisai_portal_backend::domain::group::GroupType;
use koudaisai_portal_backend::domain::group_id::GroupId;
use koudaisai_portal_backend::domain::membership::{Membership, Role};
use koudaisai_portal_backend::domain::target_specifier::TargetSpecifier;
use koudaisai_portal_backend::domain::user_id::UserId;
use koudaisai_portal_backend::infra::memory::MemoryApplication;
use koudaisai_portal_backend::infra::memory::transaction_impl::MemoryTransaction;
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

pub fn uid() -> UserId {
    UserId::new(Uuid::new_v4())
}

pub fn mem(group_id: GroupId, user_id: UserId) -> Membership {
    Membership::new(group_id, user_id, Role::Representative, &FixedClock)
}

/// 全権限を持つ管理者コンテキスト（テストでのシード用）．
fn seed_admin_ctx() -> ActorContext {
    ActorContext::Admin {
        user_id: uid(),
        claims: vec![
            "koudaisai-portal:admin:group:create".to_string(),
            "koudaisai-portal:admin:group:update".to_string(),
            "koudaisai-portal:admin:user:read".to_string(),
        ],
    }
}

/// テスト用に既存ユーザーを登録します．
pub async fn seed_user(app: &MemoryApplication, user_id: UserId) {
    let email = EmailAddress::new(format!("{}@example.com", user_id)).unwrap();
    app.user()
        .register(&seed_admin_ctx(), user_id, "Seed User".to_string(), email)
        .await
        .unwrap();
}

/// テスト用に（登録済みの）ユーザーを役職付きでグループへ追加します．
pub async fn seed_member(app: &MemoryApplication, group_id: GroupId, user_id: UserId, role: Role) {
    app.group()
        .add_member(
            &seed_admin_ctx(),
            MemoryTransaction::new(),
            group_id,
            user_id,
            role,
        )
        .await
        .unwrap();
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
        "" | "press" => GroupType::Press,
        "general" => GroupType::GeneralProject,
        "booth" => GroupType::BoothProject,
        "stage" => GroupType::StageProject,
        "labo" => GroupType::LabProject,
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
