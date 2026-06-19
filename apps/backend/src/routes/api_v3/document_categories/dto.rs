use crate::domain::document_category::DocumentCategory;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Serialize, ToSchema)]
pub struct DocumentCategoryRead {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
    emoji: Option<String>,
}

impl From<&DocumentCategory> for DocumentCategoryRead {
    fn from(cat: &DocumentCategory) -> Self {
        DocumentCategoryRead {
            id: cat.id(),
            created_at: cat.created_at(),
            updated_at: cat.updated_at(),
            title: cat.title().to_string(),
            emoji: cat.emoji().map(|s| s.to_string()),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct DocumentCategoryCreate {
    pub title: String,
    pub emoji: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct DocumentCategoryUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}
