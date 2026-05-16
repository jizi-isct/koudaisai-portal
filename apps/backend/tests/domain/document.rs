use crate::domain::common::FixedClock;
use koudaisai_portal_backend::domain::document::Document;
use koudaisai_portal_backend::domain::document::DocumentFormat;
use koudaisai_portal_backend::domain::document_category::DocumentCategory;
use koudaisai_portal_backend::domain::group_id::GroupId;
use koudaisai_portal_backend::domain::target_specifier::TargetSpecifier;
use koudaisai_portal_backend::domain::user_id::UserId;

use oauth2::url::form_urlencoded::Target;
use serde::Deserialize;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

fn make_document() -> Document {
    let format = DocumentFormat::Markdown {
        content: "Hello".to_string(),
    };

    let targets = vec![TargetSpecifier::UserNologin];

    let category = DocumentCategory::register(
        "Test Category".to_string(),
        Some("📃".to_string()),
        &FixedClock,
    )
    .unwrap();

    Document::register(
        "Test Document".to_string(),
        Some(category.id()),
        format,
        targets,
        UserId::new(Uuid::new_v4()),
        &FixedClock,
    )
    .unwrap()
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

fn parse_targets(targets: Vec<String>) -> datatest_stable::Result<Vec<TargetSpecifier>> {
    targets
        .iter()
        .map(|s| {
            TargetSpecifier::from_str(s).map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })
        })
        .collect()
}

// --- register ---
#[derive(Deserialize)]
struct RegisterCase {
    title: String,
    format: TestDocumentFormat,
    targets: Vec<String>,
    ok: bool,
}

pub fn test_register(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    // registerにcategoryに対する検証がないからチェックする必要なし
    let category = DocumentCategory::register(
        "Test Category".to_string(),
        Some("📃".to_string()),
        &FixedClock,
    )
    .unwrap();

    let c: RegisterCase = serde_json::from_str(&contents)?;

    let format: DocumentFormat = c.format.clone().into();
    let targets = parse_targets(c.targets.clone())?;

    let result = Document::register(
        c.title.clone(),
        Some(category.id()),
        format,
        targets,
        UserId::new(Uuid::new_v4()),
        &FixedClock,
    );

    if c.ok {
        match &c.format.into() {
            DocumentFormat::Markdown { content } => {
                assert!(
                    result.is_ok(),
                    "expected Ok for title={:?} format=Markdown (content={:?}) targets={:?}",
                    c.title,
                    content,
                    c.targets
                );
            }
            DocumentFormat::Pdf {
                file_key,
                file_name,
            }
            | DocumentFormat::Misc {
                file_key,
                file_name,
            } => {
                assert!(
                    result.is_ok(),
                    "expected Ok for title={:?} format=PDF/Misc (file_key={:?}, file_name={:?}) targets={:?}",
                    c.title,
                    file_key,
                    file_name,
                    c.targets
                );
            }
        }
    } else {
        match &c.format.into() {
            DocumentFormat::Markdown { content } => {
                assert!(
                    result.is_err(),
                    "expected Err for title={:?} format=Markdown (content={:?}) targets={:?}",
                    c.title,
                    content,
                    c.targets
                );
            }
            DocumentFormat::Pdf {
                file_key,
                file_name,
            }
            | DocumentFormat::Misc {
                file_key,
                file_name,
            } => {
                assert!(
                    result.is_err(),
                    "expected Err for title={:?} format=PDF/Misc (file_key={:?}, file_name={:?}) targets={:?}",
                    c.title,
                    file_key,
                    file_name,
                    c.targets
                );
            }
        }
    }
    Ok(())
}

// --- change_title ---
#[derive(Deserialize)]
struct ChangeTitleCase {
    new_title: String,
    ok: bool,
}

pub fn test_change_title(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: ChangeTitleCase = serde_json::from_str(&contents)?;
    let mut doc = make_document();
    let result = doc.change_title(
        c.new_title.clone(),
        UserId::new(Uuid::new_v4()),
        &FixedClock,
    );
    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_title);
        assert_eq!(doc.title(), c.new_title.as_str());
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_title);
    }
    Ok(())
}

// --- change_category ---
#[derive(Deserialize)]
struct ChangeCategoryCase {
    new_category: Option<Uuid>,
    ok: bool,
}

pub fn test_change_category(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: ChangeCategoryCase = serde_json::from_str(&contents)?;
    let mut doc = make_document();
    let result = doc.change_category(c.new_category, UserId::new(Uuid::new_v4()), &FixedClock);

    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_category);
        assert_eq!(doc.category(), c.new_category);
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_category);
    }
    Ok(())
}

// --- change_format ---
#[derive(Deserialize)]
struct ChangeFormatCase {
    new_format: TestDocumentFormat,
    ok: bool,
}

pub fn test_change_format(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: ChangeFormatCase = serde_json::from_str(&contents)?;
    let mut doc = make_document();
    let result = doc.change_format(
        c.new_format.clone().into(),
        UserId::new(Uuid::new_v4()),
        &FixedClock,
    );

    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_format);
        assert_eq!(doc.format(), &c.new_format.into());
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_format);
    }
    Ok(())
}

// --- change_targets ---
#[derive(Deserialize)]
struct ChangeTargetsCase {
    new_targets: Vec<String>,
    ok: bool,
}

pub fn test_change_targets(_path: &Path, contents: String) -> datatest_stable::Result<()> {
    let c: ChangeTargetsCase = serde_json::from_str(&contents)?;
    let mut doc = make_document();

    // パース失敗の可能性もある
    let parsed_targets = parse_targets(c.new_targets.clone());

    if c.ok == false {
        if parsed_targets.is_err() {
            return Ok(()); // 失敗ケースはparse失敗でも良い
        }
    }

    let targets = parsed_targets?;

    let result = doc.change_targets(targets, UserId::new(Uuid::new_v4()), &FixedClock);

    if c.ok {
        assert!(result.is_ok(), "expected Ok for {:?}", c.new_targets);
        assert_eq!(doc.targets(), parse_targets(c.new_targets)?);
    } else {
        assert!(result.is_err(), "expected Err for {:?}", c.new_targets);
    }
    Ok(())
}
