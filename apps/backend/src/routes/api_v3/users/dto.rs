use crate::domain::user::{User, UserStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub m_address: String,
}

/// `status` フィールドを判別子に持つ internally-tagged enum。
/// flatten 時に `{"status": "registered", ..}` のように展開される
/// (兄弟の `FormType` / `ApprovalRequestStatus` 等と同じ表現)。
#[derive(ToSchema, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserReadStatus {
    Registered,
    Active,
    Deactivated {
        deactivated_at: DateTime<Utc>,
        reason: String,
    },
}

impl From<&UserStatus> for UserReadStatus {
    fn from(s: &UserStatus) -> Self {
        match s {
            UserStatus::Registered => UserReadStatus::Registered,
            UserStatus::Active { .. } => UserReadStatus::Active,
            UserStatus::Deactivated {
                deactivated_at,
                reason,
                ..
            } => UserReadStatus::Deactivated {
                deactivated_at: *deactivated_at,
                reason: reason.clone(),
            },
        }
    }
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

impl From<&User> for UserRead {
    fn from(u: &User) -> Self {
        UserRead {
            id: u.id().into(),
            created_at: *u.created_at(),
            updated_at: *u.updated_at(),
            name: u.name().to_string(),
            m_address: u.m_address().address.clone(),
            status: u.status().into(),
        }
    }
}

/// `POST /users` でユーザーを新規作成したときのレスポンス。
/// 作成された(`Registered`)ユーザーが初回ログインで有効化するための
/// activation token を含む（このトークンは作成時にのみ返却される）。
#[derive(ToSchema, Serialize)]
pub struct UserCreated {
    #[serde(flatten)]
    pub user: UserRead,
    pub activation_token: String,
}

/// `POST /users/{id}/m_address` のリクエストボディ。
#[derive(ToSchema, Deserialize)]
pub struct MAddressUpdate {
    pub m_address: String,
}

/// m アドレス変更時に再発行される activation token を含むレスポンス。
#[derive(ToSchema, Serialize)]
pub struct MAddressUpdated {
    pub activation_token: String,
}

/// `PATCH /users/{id}` のリクエストボディ。氏名のみ変更可能
/// (m アドレス変更は専用エンドポイント `POST /users/{id}/m_address`)。
#[derive(ToSchema, Deserialize)]
pub struct UserUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
