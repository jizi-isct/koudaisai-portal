use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::models::email_address::EmailAddress;
use crate::domain::models::password_hash::PasswordHash;
use crate::domain::models::user_id::UserId;

pub enum UserStatus {
    Active,
    Inactive,
    Deleted,
}

pub struct User {
    pub id: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub m_address: EmailAddress,
    pub group_id: String,
    pub password_hash: PasswordHash,
    pub password_updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: UserId, name: String, m_address: EmailAddress, group_id: String, password_hash: PasswordHash) -> Self {
        Self {
            id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
            m_address,
            group_id,
            password_hash,
            password_updated_at: Utc::now(),
        }
    }
    
    
}
