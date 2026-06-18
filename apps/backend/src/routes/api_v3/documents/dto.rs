use super::super::document_categories::DocumentCategoryRead;
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

#[derive(Serialize, ToSchema)]
pub struct DocumentRead {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Uuid,
    title: String,
    category: Option<Uuid>,
    #[schema(value_type = Vec<String>)]
    targets: Vec<TargetSpecifier>,
    #[serde(flatten)]
    format: DocumentFormat,
}

/// `GET /documents/by-category` の 1 カテゴリ分のエントリ。
/// `category` が `None` のときは未分類ドキュメントを表す。
#[derive(Serialize, ToSchema)]
pub struct DocumentsByCategoryEntry {
    category: Option<DocumentCategoryRead>,
    documents: Vec<DocumentRead>,
}

#[derive(Deserialize, ToSchema)]
pub struct DocumentCreate {
    title: String,
    category: Option<Uuid>,
    #[schema(value_type = Vec<String>)]
    targets: Vec<TargetSpecifier>,
    #[serde(flatten)]
    format: DocumentFormat,
}

#[derive(Deserialize, ToSchema)]
pub struct DocumentUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Uuid>)]
    category: Option<Option<Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>)]
    targets: Option<Vec<TargetSpecifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    format: Option<DocumentFormat>,
}
