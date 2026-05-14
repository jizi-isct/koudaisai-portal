use crate::application::common::{build_actor, uid, ActorSpec};
use koudaisai_portal_backend::application::error::{ApplicationOperationError, DeleteError, UpdateError};
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::document_category::DocumentCategory;
use koudaisai_portal_backend::infra::memory::MemoryApplication;
use chrono::Utc;
use serde::Deserialize;
use std::path::Path;
use uuid::Uuid;

fn make_app() -> MemoryApplication {
    MemoryApplication::new_memory_app(Utc::now())
}

fn admin_ctx() -> ActorContext {
    ActorContext::Admin {
        user_id: uid(),
        claims: vec!["koudaisai-portal:admin:document-category:create".to_string()],
    }
}

async fn seed_category(app: &MemoryApplication) -> DocumentCategory {
    let ctx = admin_ctx();
    app.document_category()
        .create(&ctx, "Test Category".to_string(), Some("📃".to_string()))
        .await
        .unwrap()
}

// --- get_all ---

#[derive(Deserialize)]
struct GetAllCase {
    actor: ActorSpec,
    seed_count: usize,
    expected: String,
}

pub fn test_get_all(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetAllCase = serde_json::from_str(&contents)?;
        let app = make_app();

        for _ in 0..c.seed_count {
            seed_category(&app).await;
        }

        let (_, ctx) = build_actor(c.actor);
        let result = app.document_category().get_all(&ctx).await;

        match c.expected.as_str() {
            "ok" => {
                let categories = result.expect("expected Ok");
                assert_eq!(categories.len(), c.seed_count);
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
    category_exists: bool,
    expected: String,
}

pub fn test_get_by_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetByIdCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let category_id = if c.category_exists {
            let cat = seed_category(&app).await;
            cat.id()
        } else {
            Uuid::new_v4()
        };

        let (_, ctx) = build_actor(c.actor);
        let result = app.document_category().get_by_id(&ctx, category_id).await;

        match c.expected.as_str() {
            "some" => {
                let opt = result.expect("expected Ok");
                assert!(opt.is_some(), "expected Some");
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

// --- update ---

#[derive(Deserialize)]
struct UpdateCase {
    actor: ActorSpec,
    category_exists: bool,
    new_title: Option<String>,
    #[serde(default)]
    update_emoji: bool,
    new_emoji: Option<String>,
    expected: String,
}

pub fn test_update(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: UpdateCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let category_id = if c.category_exists {
            let cat = seed_category(&app).await;
            cat.id()
        } else {
            Uuid::new_v4()
        };

        let emoji_update: Option<Option<String>> = if c.update_emoji {
            Some(c.new_emoji.clone())
        } else {
            None
        };

        let (_, ctx) = build_actor(c.actor);
        let result = app
            .document_category()
            .update_document_category(&ctx, category_id, c.new_title.clone(), emoji_update)
            .await;

        match c.expected.as_str() {
            "ok" => {
                assert!(result.is_ok(), "expected Ok, got {:?}", result);
            }
            "unauthorized" => {
                assert!(matches!(result, Err(ApplicationOperationError::Unauthorized)));
            }
            "not_found" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::OperationFailed(UpdateError::NotFound))
                ));
            }
            e => panic!("unknown expected: {e}"),
        }
        Ok(())
    })
}

// --- delete ---

#[derive(Deserialize)]
struct DeleteCase {
    actor: ActorSpec,
    category_exists: bool,
    expected: String,
}

pub fn test_delete(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: DeleteCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let category_id = if c.category_exists {
            let cat = seed_category(&app).await;
            cat.id()
        } else {
            Uuid::new_v4()
        };

        let (_, ctx) = build_actor(c.actor);
        let result = app.document_category().delete_document_category(&ctx, category_id).await;

        match c.expected.as_str() {
            "ok" => {
                assert!(result.is_ok(), "expected Ok, got {:?}", result);
            }
            "unauthorized" => {
                assert!(matches!(result, Err(ApplicationOperationError::Unauthorized)));
            }
            "not_found" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::OperationFailed(DeleteError::NotFound))
                ));
            }
            e => panic!("unknown expected: {e}"),
        }
        Ok(())
    })
}
