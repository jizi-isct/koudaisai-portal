use crate::application::common::{ActorSpec, build_actor, uid};
use chrono::Utc;
use koudaisai_portal_backend::application::error::{
    ApplicationOperationError, DeleteError, UpdateError,
};
use koudaisai_portal_backend::domain::actor_ctx::ActorContext;
use koudaisai_portal_backend::domain::document::Document;
use koudaisai_portal_backend::domain::document::DocumentFormat;
use koudaisai_portal_backend::domain::target_specifier::TargetSpecifier;
use koudaisai_portal_backend::infra::memory::MemoryApplication;
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
        claims: vec!["koudaisai-portal:admin:document:create".to_string()],
    }
}

async fn seed_document(app: &MemoryApplication) -> Document {
    let ctx = admin_ctx();
    app.document()
        .create(
            &ctx,
            "Test Document".to_string(),
            Some(Uuid::new_v4()),
            vec![TargetSpecifier::GroupTypeProjectGeneral],
            DocumentFormat::Markdown {
                content: "Test Content".to_string(),
            },
        )
        .await
        .unwrap()
}

// --- create ---
#[derive(Deserialize)]
struct CreateCase {
    actor: ActorSpec,
    title: String,
    category: Option<Uuid>,
    targets: Vec<String>,
    format: TestDocumentFormat,
    expected: String,
}

pub fn test_create(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: CreateCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let targets: Vec<TargetSpecifier> = c
            .targets
            .iter()
            .map(|s| TargetSpecifier::from_str(s).unwrap())
            .collect();

        let (_, ctx) = build_actor(c.actor);
        let result = app
            .document()
            .create(&ctx, c.title, c.category, targets, c.format.into())
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
            e => panic!("unknown expected: {e}"),
        }
        Ok(())
    })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")] // JSONではバリアント名をスネークケースにする
enum TestDocumentFormat {
    Markdown { content: String },
    Pdf { file_key: String, file_name: String },
    Misc { file_key: String, file_name: String },
}

impl From<TestDocumentFormat> for DocumentFormat {
    fn from(v: TestDocumentFormat) -> Self {
        match v {
            TestDocumentFormat::Markdown { content } => DocumentFormat::Markdown { content },
            TestDocumentFormat::Pdf {
                file_key,
                file_name,
            } => DocumentFormat::Pdf {
                file_key,
                file_name,
            },
            TestDocumentFormat::Misc {
                file_key,
                file_name,
            } => DocumentFormat::Misc {
                file_key,
                file_name,
            },
        }
    }
}
// --- get_all ---

#[derive(Deserialize)]
struct GetAllCase {
    actor: ActorSpec,
    seed_count: usize,
    expected_count: usize,
    expected: String,
}

pub fn test_get_all(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetAllCase = serde_json::from_str(&contents)?;
        let app = make_app();

        for _ in 0..c.seed_count {
            seed_document(&app).await;
        }

        let (_, ctx) = build_actor(c.actor);
        let result = app.document().get_all(&ctx).await;

        match c.expected.as_str() {
            "ok" => {
                let documents = result.expect("expected Ok");
                assert_eq!(documents.len(), c.expected_count);
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

// --- get_by_category ---
#[derive(Deserialize)]
struct GetByCategoryCase {
    actor: ActorSpec,
    seed_count: usize,
    expected_count: usize,
    expected: String,
}

pub fn test_get_by_category(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetByCategoryCase = serde_json::from_str(&contents)?;
        let app = make_app();

        for _ in 0..c.seed_count {
            seed_document(&app).await;
        }

        let (_, ctx) = build_actor(c.actor);
        let result = app.document().get_by_category(&ctx).await;

        match c.expected.as_str() {
            "ok" => {
                let grouped = result.expect("expected Ok");
                let visible_count: usize = grouped.values().map(|docs| docs.len()).sum(); // 何個見えてる？
                assert_eq!(visible_count, c.expected_count);
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
    document_exists: bool,
    expected: String,
}

pub fn test_get_by_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetByIdCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let document_id = if c.document_exists {
            let doc = seed_document(&app).await;
            doc.id()
        } else {
            Uuid::new_v4()
        };

        let (_, ctx) = build_actor(c.actor); // UserIdは捨てる
        let result = app.document().get_by_id(&ctx, document_id).await;

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
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::Unauthorized)
                ));
            }
            e => panic!("unknown expecetd: {e}"),
        }
        Ok(())
    })
}

// --- update ---
#[derive(Deserialize)]
struct UpdateCase {
    actor: ActorSpec,
    document_exists: bool,
    new_title: Option<String>,
    update_category: bool,
    new_category: Option<Uuid>,
    new_format: Option<TestDocumentFormat>,
    new_targets: Option<Vec<String>>,
    expected: String,
}

pub fn test_update(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: UpdateCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let document_id = if c.document_exists {
            let doc = seed_document(&app).await;
            doc.id()
        } else {
            Uuid::new_v4()
        };

        let category_update: Option<Option<Uuid>> = if c.update_category {
            Some(c.new_category.clone())
        } else {
            None
        };

        let format_update: Option<DocumentFormat> = match c.new_format.clone() {
            Some(test_formats) => Some(test_formats.into()),
            None => None,
        };

        let targets_update: Option<Vec<TargetSpecifier>> = match c.new_targets.clone() {
            Some(targets) => Some(
                targets
                    .into_iter()
                    .map(|s| TargetSpecifier::from_str(&s).unwrap())
                    .collect(),
            ),
            None => None,
        };

        let (_, ctx) = build_actor(c.actor);
        let result = app
            .document()
            .update_document(
                &ctx,
                document_id,
                c.new_title.clone(),
                category_update,
                format_update,
                targets_update,
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
            e => panic!("unknown expected: {e}"),
        }

        Ok(())
    })
}

// --- delete ---

#[derive(Deserialize)]
struct DeleteCase {
    actor: ActorSpec,
    document_exists: bool,
    expected: String,
}

pub fn test_delete(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: DeleteCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let document_id = if c.document_exists {
            let doc = seed_document(&app).await;
            doc.id()
        } else {
            Uuid::new_v4()
        };

        let (_, ctx) = build_actor(c.actor);
        let result = app.document().delete_document(&ctx, document_id).await;

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
                ))
            }
            e => panic!("unknown expected: {e}"),
        }

        Ok(())
    })
}
