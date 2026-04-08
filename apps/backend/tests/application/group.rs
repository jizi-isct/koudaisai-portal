use crate::application::common::{build_actor, uid, ActorSpec};
use crate::domain::common::FixedClock;
use koudaisai_portal_backend::application::error::{ApplicationOperationError, ApplicationSequentialOperationError};
use koudaisai_portal_backend::application::ports::repositories::group_repo::GroupRepo;
use koudaisai_portal_backend::application::ports::repositories::membership_repo::MembershipRepo;
use koudaisai_portal_backend::domain::group::{Group, GroupType};
use koudaisai_portal_backend::domain::group_id::GroupId;
use koudaisai_portal_backend::domain::membership::Membership;
use koudaisai_portal_backend::infra::memory::MemoryApplication;
use koudaisai_portal_backend::infra::memory::transaction_impl::MemoryTransaction;
use chrono::Utc;
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;

fn make_app() -> MemoryApplication {
    MemoryApplication::new_memory_app(Utc::now())
}

// --- get_all ---

#[derive(Deserialize)]
struct GetAllCase {
    actor: ActorSpec,
    seed_group_count: usize,
    expected: String,
}

pub fn test_get_all(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetAllCase = serde_json::from_str(&contents)?;
        let app = make_app();

        for i in 0..c.seed_group_count {
            let group_id = GroupId::new('P', (i + 1) as u16).unwrap();
            let group = Group::register(
                group_id,
                format!("Test Group {}", i + 1),
                GroupType::Press { representative: uid() },
                &FixedClock,
            )
            .unwrap();
            app.group_repo().insert(group).await.unwrap();
        }

        let (_, ctx) = build_actor(c.actor);
        let result = app.group().get_all(&ctx).await;

        match c.expected.as_str() {
            "ok" => {
                let groups = result.expect("expected Ok");
                assert_eq!(groups.len(), c.seed_group_count);
            }
            "unauthorized" => {
                assert!(matches!(result, Err(ApplicationOperationError::Unauthorized)));
            }
            e => panic!("unknown expected: {e}"),
        }
        Ok(())
    })
}

// --- get_by_id ---

#[derive(Deserialize)]
struct GetByIdCase {
    actor: ActorSpec,
    group_id: String,
    group_exists: bool,
    #[serde(default)]
    seed_actor_membership: bool,
    expected: String,
}

pub fn test_get_by_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetByIdCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let group_id = GroupId::from_str(&c.group_id).unwrap();

        if c.group_exists {
            let rep_id = uid();
            let group = Group::register(
                group_id,
                "Test Group".to_string(),
                GroupType::Press { representative: rep_id },
                &FixedClock,
            )
            .unwrap();
            app.group_repo().insert(group).await.unwrap();
        }

        let (actor_uid, ctx) = build_actor(c.actor);

        if c.seed_actor_membership {
            let membership = Membership::new(group_id, actor_uid, &FixedClock);
            app.membership_repo().insert(membership).await.unwrap();
        }

        let result = app.group().get_by_id(&ctx, group_id).await;

        match c.expected.as_str() {
            "some" => {
                let opt = result.expect("expected Ok");
                assert!(opt.is_some(), "expected Some");
                assert_eq!(opt.unwrap().id(), group_id);
            }
            "none" => {
                let opt = result.expect("expected Ok");
                assert!(opt.is_none(), "expected None");
            }
            "unauthorized" => {
                assert!(matches!(result, Err(ApplicationOperationError::Unauthorized)));
            }
            e => panic!("unknown expected: {e}"),
        }
        Ok(())
    })
}

// --- create_group ---

#[derive(Deserialize)]
struct CreateGroupCase {
    actor: ActorSpec,
    group_id: String,
    group_name: String,
    expected: String,
}

pub fn test_create_group(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: CreateGroupCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let group_id = GroupId::from_str(&c.group_id).unwrap();
        let rep_id = uid();
        let group_type = GroupType::Press { representative: rep_id };
        let tx = MemoryTransaction::new();

        let (_, ctx) = build_actor(c.actor);
        let result = app
            .group()
            .create_group(&ctx, tx, group_id, c.group_name.clone(), group_type)
            .await;

        match c.expected.as_str() {
            "ok" => {
                assert!(result.is_ok(), "expected Ok, got {:?}", result);
                let saved = app.group_repo().find_by_id(group_id).await.unwrap();
                assert!(saved.is_some());
                assert_eq!(saved.unwrap().name(), c.group_name);
                let memberships = app.membership_repo().find_by_group_id(group_id).await.unwrap();
                assert_eq!(memberships.len(), 1);
                assert_eq!(memberships[0].user_id(), rep_id);
            }
            "unauthorized" => {
                assert!(matches!(
                    result,
                    Err(ApplicationSequentialOperationError::Unauthorized)
                ));
            }
            "invalid_input" => {
                assert!(matches!(
                    result,
                    Err(ApplicationSequentialOperationError::InvalidInput(_))
                ));
            }
            e => panic!("unknown expected: {e}"),
        }
        Ok(())
    })
}