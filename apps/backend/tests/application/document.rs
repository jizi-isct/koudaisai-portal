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
        name: "テストユーザー".to_string(),
        user_id: uid(),
        claims: vec![
            "koudaisai-portal:admin:document:read".to_string(),
            "koudaisai-portal:admin:document:create".to_string(),
        ],
    }
}

async fn seed_document(app: &MemoryApplication, targets: Vec<TargetSpecifier>) -> Document {
    let ctx = admin_ctx();
    app.document()
        .create(
            &ctx,
            "Test Document".to_string(),
            Some(Uuid::new_v4()),
            targets,
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
        let expected_title = c.title.clone();
        let expected_category = c.category;
        let expected_targets = targets.clone();
        let expected_format: DocumentFormat = c.format.clone().into();

        let result = app
            .document()
            .create(&ctx, c.title, c.category, targets, c.format.into())
            .await;

        match c.expected.as_str() {
            "ok" => {
                let created = result.expect("expected create Ok");

                // read back (永続化確認)
                let read_back = app
                    .document()
                    .get_by_id(&admin_ctx(), created.id())
                    .await
                    .expect("expected get_by_id Ok");

                let persisted = read_back.expect("created document should exist (Some)");

                assert_eq!(persisted.title(), expected_title.as_str());
                assert_eq!(persisted.category(), expected_category);
                assert_eq!(persisted.targets(), expected_targets.as_slice());
                assert_eq!(persisted.format(), &expected_format);
            }
            "unauthorized" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::Unauthorized)
                ));

                let admin = admin_ctx(); // 全件取得

                let read_back = app
                    .document()
                    .get_all(&admin)
                    .await
                    .expect("expected get_all Ok");

                assert_eq!(read_back.len(), 0);
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
    #[serde(default)]
    seed_targets: Vec<String>,
    expected_count: usize,
    expected: String,
}

pub fn test_get_all(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetAllCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let targets = if c.seed_targets.is_empty() {
            vec![TargetSpecifier::GroupTypeProjectGeneral]
        } else {
            c.seed_targets
                .iter()
                .map(|s| TargetSpecifier::from_str(s).unwrap())
                .collect()
        };

        for _ in 0..c.seed_count {
            seed_document(&app, targets.clone()).await;
        }

        let (_, ctx) = build_actor(c.actor);
        let result = app.document().get_all(&ctx).await;

        match c.expected.as_str() {
            "ok" => {
                let documents = result.expect("expected Ok");
                assert_eq!(documents.len(), c.expected_count);
            }
            "unauthorized" => {
                let documents = result.expect("expected Ok");
                assert_eq!(documents.len(), c.expected_count);
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
    #[serde(default)]
    seed_targets: Vec<String>,
    expected_count: usize,
    expected: String,
}

pub fn test_get_by_category(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetByCategoryCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let targets = if c.seed_targets.is_empty() {
            vec![TargetSpecifier::GroupTypeProjectGeneral]
        } else {
            c.seed_targets
                .iter()
                .map(|s| TargetSpecifier::from_str(s).unwrap())
                .collect()
        };

        for _ in 0..c.seed_count {
            seed_document(&app, targets.clone()).await;
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
                let documents = result.expect("expected Ok");
                assert_eq!(documents.len(), c.expected_count);
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
    #[serde(default)]
    document_targets: Vec<String>,
    expected: String,
}

pub fn test_get_by_id(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    crate::application::common::run(async {
        let c: GetByIdCase = serde_json::from_str(&contents)?;
        let app = make_app();

        let document_id = if c.document_exists {
            let targets = if c.document_targets.is_empty() {
                vec![TargetSpecifier::GroupTypeProjectGeneral]
            } else {
                c.document_targets
                    .iter()
                    .map(|s| TargetSpecifier::from_str(s).unwrap())
                    .collect()
            };
            let doc = seed_document(&app, targets).await;
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

        let before_doc = seed_document(&app, vec![TargetSpecifier::GroupTypeProjectGeneral]).await;
        let document_id = if c.document_exists {
            before_doc.id()
        } else {
            Uuid::new_v4()
        };

        let category_update: Option<Option<Uuid>> = if c.update_category {
            Some(c.new_category)
        } else {
            None
        };

        let format_update: Option<DocumentFormat> = c.new_format.clone().map(|test_formats| test_formats.into());

        let targets_update: Option<Vec<TargetSpecifier>> = c.new_targets.clone().map(|targets| targets
                    .into_iter()
                    .map(|s| TargetSpecifier::from_str(&s).unwrap())
                    .collect());

        let (_, ctx) = build_actor(c.actor);
        let result = app
            .document()
            .update_document(
                &ctx,
                document_id,
                c.new_title.clone(),
                category_update,
                format_update.clone(),
                targets_update.clone(),
            )
            .await;

        match c.expected.as_str() {
            "ok" => {
                result.expect("expected update Ok");

                let after_doc = app
                    .document()
                    .get_by_id(&admin_ctx(), document_id)
                    .await
                    .expect("expected get_all Ok")
                    .expect("Some");

                if let Some(title) = c.new_title {
                    assert_eq!(title, after_doc.title());
                } else {
                    assert_eq!(before_doc.title(), after_doc.title());
                }

                if let Some(category) = category_update {
                    assert_eq!(category, after_doc.category());
                } else {
                    assert_eq!(before_doc.category(), after_doc.category());
                }

                if let Some(format) = format_update {
                    assert_eq!(&format, after_doc.format());
                } else {
                    assert_eq!(before_doc.format(), after_doc.format());
                }

                if let Some(targets) = targets_update {
                    assert_eq!(&targets, after_doc.targets());
                } else {
                    assert_eq!(before_doc.targets(), after_doc.targets());
                }
            }
            "unauthorized" => {
                let after_doc = app
                    .document()
                    .get_by_id(&admin_ctx(), document_id)
                    .await
                    .expect("expected get_all Ok")
                    .expect("Some");

                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::Unauthorized)
                ));
                assert_eq!(before_doc.title(), after_doc.title());
                assert_eq!(before_doc.category(), after_doc.category());
                assert_eq!(before_doc.format(), after_doc.format());
                assert_eq!(before_doc.targets(), after_doc.targets());
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

        let document = seed_document(&app, vec![TargetSpecifier::GroupTypeProjectGeneral]).await;

        let document_id = if c.document_exists {
            document.id()
        } else {
            Uuid::new_v4()
        };

        let (_, ctx) = build_actor(c.actor);
        let result = app.document().delete_document(&ctx, document_id).await;

        match c.expected.as_str() {
            "ok" => {
                result.expect("expected delete Ok");

                let read_back = app
                    .document()
                    .get_all(&admin_ctx())
                    .await
                    .expect("expected get_all Ok");
                assert_eq!(read_back.len(), 0);
            }
            "unauthorized" => {
                assert!(matches!(
                    result,
                    Err(ApplicationOperationError::Unauthorized)
                ));
                let read_back = app
                    .document()
                    .get_by_id(&admin_ctx(), document_id)
                    .await
                    .expect("expected get_all Ok")
                    .expect("Some");

                assert_eq!(document.id(), read_back.id());
                assert_eq!(document.title(), read_back.title());
                assert_eq!(document.category(), read_back.category());
                assert_eq!(document.targets(), read_back.targets());
                assert_eq!(document.format(), read_back.format());
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
