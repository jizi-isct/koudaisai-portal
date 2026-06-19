use super::super::document_categories::DocumentCategoryRead;
use crate::domain::document::{Document, DocumentFormat as DomainDocumentFormat};
use crate::domain::target_specifier::TargetSpecifier;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "format", rename_all = "snake_case")]
pub enum DocumentFormat {
    Markdown { content: String },
    Pdf { file_key: String, file_name: String },
    Misc { file_key: String, file_name: String },
}

impl From<&DomainDocumentFormat> for DocumentFormat {
    fn from(f: &DomainDocumentFormat) -> Self {
        match f {
            DomainDocumentFormat::Markdown { content } => DocumentFormat::Markdown {
                content: content.clone(),
            },
            DomainDocumentFormat::Pdf {
                file_key,
                file_name,
            } => DocumentFormat::Pdf {
                file_key: file_key.clone(),
                file_name: file_name.clone(),
            },
            DomainDocumentFormat::Misc {
                file_key,
                file_name,
            } => DocumentFormat::Misc {
                file_key: file_key.clone(),
                file_name: file_name.clone(),
            },
        }
    }
}

impl From<DocumentFormat> for DomainDocumentFormat {
    fn from(f: DocumentFormat) -> Self {
        match f {
            DocumentFormat::Markdown { content } => DomainDocumentFormat::Markdown { content },
            DocumentFormat::Pdf {
                file_key,
                file_name,
            } => DomainDocumentFormat::Pdf {
                file_key,
                file_name,
            },
            DocumentFormat::Misc {
                file_key,
                file_name,
            } => DomainDocumentFormat::Misc {
                file_key,
                file_name,
            },
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct DocumentRead {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub title: String,
    pub category: Option<Uuid>,
    #[schema(value_type = Vec<String>)]
    pub targets: Vec<TargetSpecifier>,
    #[serde(flatten)]
    pub format: DocumentFormat,
}

impl From<&Document> for DocumentRead {
    fn from(d: &Document) -> Self {
        DocumentRead {
            id: d.id(),
            created_at: d.created_at(),
            updated_at: d.updated_at(),
            created_by: d.created_by().as_uuid(),
            title: d.title().to_string(),
            category: d.category(),
            targets: d.targets().to_vec(),
            format: d.format().into(),
        }
    }
}

/// `GET /documents/by-category` の 1 カテゴリ分のエントリ。
/// `category` が `None` のときは未分類ドキュメントを表す。
#[derive(Serialize, ToSchema)]
pub struct DocumentsByCategoryEntry {
    pub category: Option<DocumentCategoryRead>,
    pub documents: Vec<DocumentRead>,
}

#[derive(Deserialize, ToSchema)]
pub struct DocumentCreate {
    pub title: String,
    pub category: Option<Uuid>,
    #[schema(value_type = Vec<String>)]
    pub targets: Vec<TargetSpecifier>,
    #[serde(flatten)]
    pub format: DocumentFormat,
}

#[derive(Deserialize, ToSchema)]
pub struct DocumentUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Uuid>)]
    pub category: Option<Option<Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>)]
    pub targets: Option<Vec<TargetSpecifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<DocumentFormat>,
}
