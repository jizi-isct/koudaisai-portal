use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub m_address: String,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserReadStatus {
    StatusRegistered,
    StatusActive,
    StatusDeactivated {
        deactivated_at: DateTime<Utc>,
        reason: String,
    },
}

#[derive(ToSchema, Serialize)]
pub struct UserRead {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub m_address: String,
    #[serde(flatten)]
    pub status: UserReadStatus,
}

#[derive(ToSchema, Deserialize)]
pub struct UserUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub m_address: Option<String>,
}
