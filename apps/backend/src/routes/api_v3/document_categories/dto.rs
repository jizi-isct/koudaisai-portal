use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct DocumentCategoryRead {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
    emoji: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct DocumentCategoryCreate {
    title: String,
    emoji: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct DocumentCategoryUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    emoji: Option<String>,
}
