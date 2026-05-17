use crate::application::common::{ActorSpec, build_actor, parse_target, uid};
use chrono::Utc;
use koudaisai_portal_backend::application::error::{
    ApplicationOperationError, DeleteError, UpdateError,
};
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::form::FormType;
use koudaisai_portal_backend::domain::form_id::FormId;
use koudaisai_portal_backend::domain::target_specifier::TargetSpecifier;
use koudaisai_portal_backend::infra::memory::MemoryApplication;
use serde::Deserialize;
use std::path::Path;
use uuid::Uuid;

fn make_app() -> MemoryApplication {
    MemoryApplication::new_memory_app(Utc::now())
}

fn admin_ctx() -> ActorContext {
    ActorContext::Admin {
        user_id: uid(),
        claims: vec!["koudaisai-portal:admin:form:create".to_string()],
    }
}

async fn seed_form(app: &MemoryApplication, targets: Vec<TargetSpecifier>) -> FormId {
    let ctx = admin_ctx();
    app.form()
        .create(
            &ctx,
            targets,
            "Test Form".to_string(),
            "Test Summary".to_string(),
            Utc::now() + chrono::Duration::days(7),
            FormType::TypeExternal {
                form_url: "https://example.com/form".to_string(),
            },
        )
        .await
        .unwrap()
}

// --- get_all ---

#[derive(Deserialize)]
struct GetAllCase {
    actor: ActorSpec,
    form_targets: Vec<String>,
    expected_count: usize,
}

pub fn test_get_all(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetAllCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let targets: Vec<TargetSpecifier> =
            c.form_targets.iter().map(|t| parse_target(t)).collect();
        seed_form(&app, targets).await;

        let (_, ctx) = build_actor(c.actor);
        let result = app.form().get_all(&ctx).await;

        let forms = result.expect("expected Ok");
        assert_eq!(forms.len(), c.expected_count);
        Ok(())
    })
}

// --- get_by_id ---

#[derive(Deserialize)]
struct GetByIdCase {
    actor: ActorSpec,
    form_exists: bool,
    form_targets: Vec<String>,
    expected: String,
}

pub fn test_get_by_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetByIdCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let targets: Vec<TargetSpecifier> =
            c.form_targets.iter().map(|t| parse_target(t)).collect();
        let form_id = if c.form_exists {
            seed_form(&app, targets).await
        } else {
            FormId::new(Uuid::new_v4())
        };

        let (_, ctx) = build_actor(c.actor);
        let result = app.form().get_by_id(&ctx, form_id).await;

        match c.expected.as_str() {
            "some" => {
                let opt = result.expect("expected Ok");
                assert!(opt.is_some(), "expected Some");
            }
            "none" => {
                let opt = result.expect("expected Ok");
                assert!(opt.is_none(), "expected None");
            }
            e => panic!("unknown expected: {e}"),
        }
        Ok(())
    })
}

// --- create ---

#[derive(Deserialize)]
struct CreateCase {
    actor: ActorSpec,
    form_name: String,
    expected: String,
}

pub fn test_create(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: CreateCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let (_, ctx) = build_actor(c.actor);
        let result = app
            .form()
            .create(
                &ctx,
                vec![TargetSpecifier::GroupTypePress],
                c.form_name.clone(),
                "Summary".to_string(),
                Utc::now() + chrono::Duration::days(7),
                FormType::TypeExternal {
                    form_url: "https://example.com".to_string(),
                },
            )
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
            "invalid_input" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::InvalidInput(_))
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
    form_exists: bool,
    new_name: Option<String>,
    new_summary: Option<String>,
    expected: String,
}

pub fn test_update(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: UpdateCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let form_id = if c.form_exists {
            seed_form(&app, vec![TargetSpecifier::GroupTypePress]).await
        } else {
            FormId::new(Uuid::new_v4())
        };

        let (_, ctx) = build_actor(c.actor);
        let result = app
            .form()
            .update(
                &ctx,
                form_id,
                None,
                c.new_name.clone(),
                c.new_summary.clone(),
                None,
                None,
            )
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
            "invalid_input" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::InvalidInput(_))
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
    form_exists: bool,
    expected: String,
}

pub fn test_delete(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: DeleteCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let form_id = if c.form_exists {
            seed_form(&app, vec![TargetSpecifier::GroupTypePress]).await
        } else {
            FormId::new(Uuid::new_v4())
        };

        let (_, ctx) = build_actor(c.actor);
        let result = app.form().delete(&ctx, form_id).await;

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
                        DeleteError::NotFound
                    ))
                ));
            }
            e => panic!("unknown expected: {e}"),
        }
        Ok(())
    })
}
