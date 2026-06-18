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

/// `PUT /users/{id}` でユーザーを新規作成したときのレスポンス。
/// 作成された(`Registered`)ユーザーが初回ログインで有効化するための
/// activation token を含む（このトークンは作成時にのみ返却される）。
#[derive(ToSchema, Serialize)]
pub struct UserCreated {
    #[serde(flatten)]
    pub user: UserRead,
    pub activation_token: String,
}

#[derive(ToSchema, Deserialize)]
pub struct UserUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub m_address: Option<String>,
}
