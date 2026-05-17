use crate::application::common::{ActorSpec, build_actor, uid};
use crate::domain::common::FixedClock;
use chrono::Utc;
use koudaisai_portal_backend::application::error::{ApplicationOperationError, UpdateError};
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::email_address::EmailAddress;
use koudaisai_portal_backend::domain::group::GroupType;
use koudaisai_portal_backend::domain::group_id::GroupId;
use koudaisai_portal_backend::infra::memory::MemoryApplication;
use koudaisai_portal_backend::infra::memory::transaction_impl::MemoryTransaction;
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

fn make_app() -> MemoryApplication {
    MemoryApplication::new_memory_app(Utc::now())
}

fn admin_ctx() -> ActorContext {
    ActorContext::Admin {
        user_id: uid(),
        claims: vec!["koudaisai-portal:admin:user:read".to_string()],
    }
}

async fn seed_user(app: &MemoryApplication) -> koudaisai_portal_backend::domain::user::User {
    let user_id = uid();
    let email = EmailAddress::new(format!("user-{}@example.com", Uuid::new_v4())).unwrap();
    let ctx = admin_ctx();
    app.user()
        .register(&ctx, user_id, "Test User".to_string(), email)
        .await
        .unwrap()
}

// --- get_all ---

#[derive(Deserialize)]
struct GetAllCase {
    actor: ActorSpec,
    seed_user_count: usize,
    expected: String,
}

pub fn test_get_all(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetAllCase = serde_json::from_str(&contents)?;
        let app = make_app();

        for _ in 0..c.seed_user_count {
            seed_user(&app).await;
        }

        let (_, ctx) = build_actor(c.actor);
        let result = app.user().get_all(&ctx).await;

        match c.expected.as_str() {
            "ok" => {
                let users = result.expect("expected Ok");
                assert_eq!(users.len(), c.seed_user_count);
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
    target_exists: bool,
    #[serde(default)]
    target_group_ids: Vec<String>,
    expected: String,
}

pub fn test_get_by_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetByIdCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let target_user_id = uid();
        if c.target_exists {
            let email = EmailAddress::new("target@example.com".to_string()).unwrap();
            let ctx = admin_ctx();
            app.user()
                .register(&ctx, target_user_id, "Target User".to_string(), email)
                .await
                .unwrap();
        }

        for (i, gid_str) in c.target_group_ids.iter().enumerate() {
            let gid = GroupId::from_str(gid_str).unwrap();
            let group_type = GroupType::Press {
                representative: target_user_id,
            };
            let admin = ActorContext::Admin {
                user_id: uid(),
                claims: vec!["koudaisai-portal:admin:group:create".to_string()],
            };
            app.group()
                .create_group(
                    &admin,
                    MemoryTransaction::new(),
                    gid,
                    format!("group-{}", i),
                    group_type,
                )
                .await
                .unwrap();
        }

        let (_, ctx) = build_actor(c.actor);
        let result = app.user().get_by_id(&ctx, target_user_id).await;

        match c.expected.as_str() {
            "some" => {
                let opt = result.expect("expected Ok");
                assert!(opt.is_some(), "expected Some");
                assert_eq!(opt.unwrap().id(), target_user_id);
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

// --- update ---

#[derive(Deserialize)]
struct UpdateCase {
    actor: ActorSpec,
    expected: String,
}

pub fn test_update(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: UpdateCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let mut user = seed_user(&app).await;
        user.rename("Updated Name".to_string(), &FixedClock)
            .unwrap();

        let (_, ctx) = build_actor(c.actor);
        let result = app.user().update_user(&ctx, &user).await;

        match c.expected.as_str() {
            "ok" => {
                assert!(result.is_ok(), "expected Ok, got {:?}", result);
                let admin = admin_ctx();
                let saved = app.user().get_by_id(&admin, user.id()).await.unwrap();
                assert!(saved.is_some());
                assert_eq!(saved.unwrap().name(), "Updated Name");
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

// --- change_m_address ---

#[derive(Deserialize)]
struct ChangeMAddressCase {
    actor: ActorSpec,
    target_exists: bool,
    expected: String,
}

pub fn test_change_m_address(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: ChangeMAddressCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let target_user_id = uid();
        if c.target_exists {
            let email = EmailAddress::new("change-target@example.com".to_string()).unwrap();
            let ctx = admin_ctx();
            app.user()
                .register(&ctx, target_user_id, "Target User".to_string(), email)
                .await
                .unwrap();
        }

        let new_email = EmailAddress::new("new@example.com".to_string()).unwrap();
        let (_, ctx) = build_actor(c.actor);
        let result = app
            .user()
            .change_m_address(&ctx, target_user_id, new_email)
            .await;

        match c.expected.as_str() {
            "ok" => {
                assert!(result.is_ok(), "expected Ok, got {:?}", result);
            }
            "unauthorized" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::Unauthorized)
                ));
            }
            "not_found" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::OperationFailed(
                        UpdateError::NotFound
                    ))
                ));
            }
            e => panic!("unknown expected: {e}"),
        }
        Ok(())
    })
}
