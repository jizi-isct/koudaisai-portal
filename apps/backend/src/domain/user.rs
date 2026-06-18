use crate::application::ports::clock::Clock;
use crate::domain::email_address::EmailAddress;
use crate::domain::error::{FactoryError, InvalidTransitionError};
use crate::domain::password_credentials::PasswordCredentials;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};

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
        clock: C,
    ) -> Result<Self, FactoryError> {
        if name.trim().is_empty() {
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
    /// `Registered` 状態のユーザーのみアクティベート可能
    /// それ以外の状態の場合、`InvalidTransitionError` を返す
    ///
    /// `Deactivated` からの復帰はパスワードリセット経路で扱う。
    /// (旧実装は `Deactivated` アームで引数 `password_credentials` をパターン束縛で
    ///  握り潰し，古い creds を clone する不具合があったため `Registered` 限定にした。)
    pub fn activate<C: Clock>(
        &mut self,
        password_credentials: PasswordCredentials,
        clock: &C,
    ) -> Result<(), InvalidTransitionError> {
        match &self.status {
            UserStatus::Registered => {
                self.status = UserStatus::Active {
                    password_credentials,
                };
                self.updated_at = clock.now();
                Ok(())
            }
            _ => Err(InvalidTransitionError {}),
        }
    }

    /// パスワードを変更する。`Active` 状態のユーザーのみ可能。
    /// `password_credentials.changed_at` が更新され，発行済みアクセストークンの
    /// `iat` ゲート(iat >= changed_at)を失効させる。
    pub fn change_password<C: Clock>(
        &mut self,
        new_phc: String,
        clock: &C,
    ) -> Result<(), InvalidTransitionError> {
        match &mut self.status {
            UserStatus::Active {
                password_credentials,
            } => {
                password_credentials
                    .change(new_phc, clock)
                    .map_err(|_| InvalidTransitionError {})?;
            }
            _ => return Err(InvalidTransitionError {}),
        }
        self.updated_at = clock.now();
        Ok(())
    }

    /// ハッシュの透過昇格(rehash)。`Active` 状態のユーザーのみ可能。
    /// ログイン成功時に旧パラメータのハッシュを現行 argon2 へ差し替える用途。
    /// ユーザー操作ではないため `changed_at` は更新せず，`updated_at` のみ更新する。
    pub fn rehash_password<C: Clock>(
        &mut self,
        new_phc: String,
        clock: &C,
    ) -> Result<(), InvalidTransitionError> {
        match &mut self.status {
            UserStatus::Active {
                password_credentials,
            } => {
                password_credentials.rehash(new_phc);
            }
            _ => return Err(InvalidTransitionError {}),
        }
        self.updated_at = clock.now();
        Ok(())
    }

    /// 保存済みパスワード PHC を返す。`Registered`(未有効化)は未設定のため `None`。
    pub fn password_phc(&self) -> Option<&str> {
        match &self.status {
            UserStatus::Active {
                password_credentials,
            }
            | UserStatus::Deactivated {
                password_credentials,
                ..
            } => Some(password_credentials.phc()),
            UserStatus::Registered => None,
        }
    }

    /// ユーザーを無効化するメソッド
    /// `Active` 状態のユーザーのみ無効化可能
    /// それ以外の状態の場合、`InvalidTransitionError` を返す
    pub fn deactivate<C: Clock>(
        &mut self,
        reason: String,
        clock: &C,
    ) -> Result<(), InvalidTransitionError> {
        match &self.status {
            UserStatus::Active {
                password_credentials,
            } => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::email_address::EmailAddress;
    use crate::domain::user_id::UserId;
    use chrono::{Duration, TimeZone};
    use uuid::Uuid;

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    fn setup_user(clock: &MockClock) -> User {
        let id = UserId::new(Uuid::new_v4());
        let name = "Test User".to_string();
        let m_address = EmailAddress::new("test@example.com".to_string()).unwrap();
        User::register(id, name, m_address, clock).unwrap()
    }

    #[test]
    fn test_restore_success() {
        let id = UserId::new(Uuid::new_v4());
        let created_at = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let updated_at = Utc.with_ymd_and_hms(2024, 1, 1, 1, 0, 0).unwrap();
        let name = "Restored User".to_string();
        let m_address = EmailAddress::new("restored@example.com".to_string()).unwrap();
        let status = UserStatus::Registered;

        let user = User::restore(
            id,
            created_at,
            updated_at,
            name.clone(),
            m_address.clone(),
            status.clone(),
        );

        assert_eq!(user.id(), id);
        assert_eq!(user.created_at(), &created_at);
        assert_eq!(user.updated_at(), &updated_at);
        assert_eq!(user.name(), name);
        assert_eq!(user.m_address(), &m_address);
        assert_eq!(user.status(), &status);
    }

    #[test]
    fn test_change_m_address_success() {
        let initial_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock1 = MockClock { now: initial_time };
        let mut user = setup_user(&clock1);

        let update_time = initial_time + Duration::seconds(20);
        let clock2 = MockClock { now: update_time };
        let new_address = EmailAddress::new("new@example.com".to_string()).unwrap();

        user.change_m_address(new_address.clone(), &clock2);

        assert_eq!(user.m_address(), &new_address);
        assert_eq!(user.updated_at(), &update_time);
    }

    fn creds(phc: &str, clock: &MockClock) -> PasswordCredentials {
        PasswordCredentials::new(phc.to_string(), clock).unwrap()
    }

    #[test]
    fn test_activate_from_registered_sets_phc() {
        let clock = MockClock {
            now: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        };
        let mut user = setup_user(&clock);
        assert_eq!(user.password_phc(), None);

        user.activate(creds("$fake$pw", &clock), &clock).unwrap();

        assert!(matches!(user.status(), UserStatus::Active { .. }));
        assert_eq!(user.password_phc(), Some("$fake$pw"));
    }

    #[test]
    fn test_activate_twice_fails() {
        let clock = MockClock {
            now: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        };
        let mut user = setup_user(&clock);
        user.activate(creds("$fake$pw", &clock), &clock).unwrap();

        // すでに Active なので再 activate は不可。
        assert!(user.activate(creds("$fake$pw2", &clock), &clock).is_err());
        assert_eq!(user.password_phc(), Some("$fake$pw"));
    }

    #[test]
    fn test_change_password_requires_active() {
        let clock = MockClock {
            now: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        };
        let mut user = setup_user(&clock); // Registered
        assert!(
            user.change_password("$fake$new".to_string(), &clock)
                .is_err()
        );

        user.activate(creds("$fake$pw", &clock), &clock).unwrap();
        assert!(
            user.change_password("$fake$new".to_string(), &clock)
                .is_ok()
        );
        assert_eq!(user.password_phc(), Some("$fake$new"));
    }

    #[test]
    fn test_rehash_password_keeps_changed_at_but_bumps_updated_at() {
        let t0 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock0 = MockClock { now: t0 };
        let mut user = setup_user(&clock0);
        user.activate(creds("$fake$pw", &clock0), &clock0).unwrap();

        let t1 = t0 + Duration::seconds(100);
        let clock1 = MockClock { now: t1 };
        user.rehash_password("$argon2$better".to_string(), &clock1)
            .unwrap();

        assert_eq!(user.password_phc(), Some("$argon2$better"));
        assert_eq!(user.updated_at(), &t1);
        // rehash はユーザー操作ではないので changed_at は据え置き。
        match user.status() {
            UserStatus::Active {
                password_credentials,
            } => assert_eq!(password_credentials.changed_at(), &t0),
            _ => panic!("expected Active"),
        }
    }
}
