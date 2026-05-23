use crate::application::common::{ActorSpec, build_actor, uid};
use chrono::Utc;
use koudaisai_portal_backend::application::error::{
    ApplicationOperationError, ApplicationSequentialOperationError,
};
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::group::GroupType;
use koudaisai_portal_backend::domain::group_id::GroupId;
use koudaisai_portal_backend::domain::user_id::UserId;
use koudaisai_portal_backend::infra::memory::MemoryApplication;
use koudaisai_portal_backend::infra::memory::transaction_impl::MemoryTransaction;
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;

fn make_app() -> MemoryApplication {
    MemoryApplication::new_memory_app(Utc::now())
}

fn admin_create_ctx() -> ActorContext {
    ActorContext::Admin {
        user_id: uid(),
        claims: vec!["koudaisai-portal:admin:group:create".to_string()],
    }
}

fn admin_read_ctx() -> ActorContext {
    ActorContext::Admin {
        user_id: uid(),
        claims: vec!["koudaisai-portal:admin:group:read".to_string()],
    }
}

async fn seed_group(app: &MemoryApplication, group_id: GroupId, name: String, rep_id: UserId) {
    let group_type = GroupType::Press {
        representative: rep_id,
    };
    app.group()
        .create_group(
            &admin_create_ctx(),
            MemoryTransaction::new(),
            group_id,
            name,
            group_type,
        )
        .await
        .unwrap();
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
            seed_group(&app, group_id, format!("Test Group {}", i + 1), uid()).await;
        }

        let (_, ctx) = build_actor(c.actor);
        let result = app.group().get_all(&ctx).await;

        match c.expected.as_str() {
            "ok" => {
                let groups = result.expect("expected Ok");
                assert_eq!(groups.len(), c.seed_group_count);
            }
            "unauthorized" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::Unauthorized)
                ));
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

        let (actor_uid, ctx) = build_actor(c.actor);

        if c.group_exists {
            // If seed_actor_membership, use actor_uid as representative so they get a DB membership
            let rep_id = if c.seed_actor_membership {
                actor_uid
            } else {
                uid()
            };
            seed_group(&app, group_id, "Test Group".to_string(), rep_id).await;
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
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::Unauthorized)
                ));
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
        let group_type = GroupType::Press {
            representative: rep_id,
        };
        let tx = MemoryTransaction::new();

        let (_, ctx) = build_actor(c.actor);
        let result = app
            .group()
            .create_group(&ctx, tx, group_id, c.group_name.clone(), group_type)
            .await;

        match c.expected.as_str() {
            "ok" => {
                assert!(result.is_ok(), "expected Ok, got {:?}", result);
                let admin = admin_read_ctx();
                let saved = app.group().get_by_id(&admin, group_id).await.unwrap();
                assert!(saved.is_some());
                assert_eq!(saved.unwrap().name(), c.group_name);
                // Verify the representative has a membership by checking that
                // get_by_id returns Some for a context that includes rep_id in the group
                let rep_ctx = ActorContext::User {
                    user_id: rep_id,
                    memberships: vec![],
                    group_type: GroupType::Press {
                        representative: rep_id,
                    },
                };
                let visible = app.group().get_by_id(&rep_ctx, group_id).await.unwrap();
                assert!(
                    visible.is_some(),
                    "representative should be able to see their group"
                );
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
