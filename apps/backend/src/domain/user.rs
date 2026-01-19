use crate::domain::password_credentials::{PasswordCredentials};
use crate::domain::email_address::EmailAddress;
use crate::domain::error::{FactoryError, InvalidTransitionError};
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};
use crate::application::ports::clock::Clock;

/// ユーザーのステータス
/// * `Registered` - ユーザーが登録されたが、まだアクティベートされていない状態を示す
/// * `Active` - ユーザーが有効な状態であることを示す
/// * `Deactivated` - ユーザーが無効化された状態を示す
#[derive(Debug, Clone, PartialEq)]
pub enum UserStatus {
    Registered,
    Active {
        password_credentials: PasswordCredentials,
    },
    Deactivated {
        password_credentials: PasswordCredentials,
        deactivated_at: DateTime<Utc>,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: UserId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    m_address: EmailAddress,
    status: UserStatus,
}

impl User {
    /// 新規登録用コンストラクタ
    /// ユーザー登録時に使用する
    pub fn register<C: Clock>(
        id: UserId,
        name: String,
        m_address: EmailAddress,
        clock: C
    ) -> Result<Self, FactoryError> {
        if (name.trim().is_empty()) {
            return Err(FactoryError::InvalidInput("Name is empty".to_string()));
        }

        Ok(Self {
            id,
            created_at: clock.now(),
            updated_at: clock.now(),
            name,
            m_address,
            status: UserStatus::Registered,
        })
    }

    /// 復元用コンストラクタ
    /// データベースなどから復元する際に使用する
    pub fn restore(
        id: UserId,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        name: String,
        m_address: EmailAddress,
        status: UserStatus,
    ) -> Self {
        Self {
            id,
            created_at,
            updated_at,
            name,
            m_address,
            status,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rename<C: Clock>(&mut self, new_name: String, clock: C) -> Result<(), FactoryError> {
        if new_name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Name is empty".to_string()));
        }
        self.name = new_name;
        self.updated_at = clock.now();
        Ok(())
    }

    pub fn m_address(&self) -> &EmailAddress {
        &self.m_address
    }

    pub fn change_m_address<C: Clock>(&mut self, new_address: EmailAddress, clock: &C) {
        self.m_address = new_address;
        self.updated_at = clock.now();
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    /// ユーザーをアクティベートするメソッド
    /// `Registered` または `Deactivated` 状態のユーザーのみアクティベート可能
    /// それ以外の状態の場合、`InvalidTransitionError` を返す
    pub fn activate<C: Clock>(&mut self, password_credentials: PasswordCredentials, clock: &C) -> Result<(), InvalidTransitionError> {
        match &self.status {
            UserStatus::Registered => {
                self.status = UserStatus::Active {
                    password_credentials,
                };
                self.updated_at = clock.now();
                Ok(())
            },
            UserStatus::Deactivated { password_credentials, .. } => {
                self.status = UserStatus::Active {
                    password_credentials: password_credentials.clone(),
                };
                self.updated_at = clock.now();
                Ok(())
            }
            _ => Err(InvalidTransitionError {}),
        }
    }

    /// ユーザーを無効化するメソッド
    /// `Active` 状態のユーザーのみ無効化可能
    /// それ以外の状態の場合、`InvalidTransitionError` を返す
    pub fn deactivate<C: Clock>(&mut self, reason: String, clock: &C) -> Result<(), InvalidTransitionError> {
        match &self.status {
            UserStatus::Active { password_credentials } => {
                self.status = UserStatus::Deactivated {
                    password_credentials: password_credentials.clone(),
                    deactivated_at: clock.now(),
                    reason,
                };
                self.updated_at = clock.now();
                Ok(())
            }
            _ => Err(InvalidTransitionError {}),
        }
    }
}
