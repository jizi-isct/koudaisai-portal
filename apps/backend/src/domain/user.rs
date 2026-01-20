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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_id::UserId;
    use crate::domain::email_address::EmailAddress;
    use crate::domain::password_credentials::PasswordCredentials;
    use uuid::Uuid;
    use chrono::{TimeZone, Duration};

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    impl Clock for &MockClock {
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
    fn test_register_success() {
        let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock = MockClock { now };
        let id = UserId::new(Uuid::new_v4());
        let name = "Test User".to_string();
        let m_address = EmailAddress::new("test@example.com".to_string()).unwrap();

        let user = User::register(id, name.clone(), m_address.clone(), clock).unwrap();

        assert_eq!(user.id(), id);
        assert_eq!(user.name(), name);
        assert_eq!(user.m_address(), &m_address);
        assert_eq!(user.created_at(), &now);
        assert_eq!(user.updated_at(), &now);
        assert_eq!(user.status(), &UserStatus::Registered);
    }

    #[test]
    fn test_register_empty_name() {
        let now = Utc::now();
        let clock = MockClock { now };
        let id = UserId::new(Uuid::new_v4());
        let m_address = EmailAddress::new("test@example.com".to_string()).unwrap();

        let result = User::register(id, "  ".to_string(), m_address, clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
    }

    #[test]
    fn test_restore_success() {
        let id = UserId::new(Uuid::new_v4());
        let created_at = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let updated_at = Utc.with_ymd_and_hms(2024, 1, 1, 1, 0, 0).unwrap();
        let name = "Restored User".to_string();
        let m_address = EmailAddress::new("restored@example.com".to_string()).unwrap();
        let status = UserStatus::Registered;

        let user = User::restore(id, created_at, updated_at, name.clone(), m_address.clone(), status.clone());

        assert_eq!(user.id(), id);
        assert_eq!(user.created_at(), &created_at);
        assert_eq!(user.updated_at(), &updated_at);
        assert_eq!(user.name(), name);
        assert_eq!(user.m_address(), &m_address);
        assert_eq!(user.status(), &status);
    }

    #[test]
    fn test_rename_success() {
        let initial_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock1 = MockClock { now: initial_time };
        let mut user = setup_user(&clock1);

        let update_time = initial_time + Duration::seconds(10);
        let clock2 = MockClock { now: update_time };
        let new_name = "New Name".to_string();

        let result = user.rename(new_name.clone(), clock2);

        assert!(result.is_ok());
        assert_eq!(user.name(), new_name);
        assert_eq!(user.updated_at(), &update_time);
    }

    #[test]
    fn test_rename_empty_name() {
        let now = Utc::now();
        let clock = MockClock { now };
        let mut user = setup_user(&clock);

        let result = user.rename(" ".to_string(), clock);
        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
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

    #[test]
    fn test_activate_from_registered() {
        let initial_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock1 = MockClock { now: initial_time };
        let mut user = setup_user(&clock1);

        let activate_time = initial_time + Duration::seconds(30);
        let clock2 = MockClock { now: activate_time };
        let creds = PasswordCredentials::new("phc".to_string(), &clock2).unwrap();

        let result = user.activate(creds.clone(), &clock2);

        assert!(result.is_ok());
        if let UserStatus::Active { password_credentials } = user.status() {
            assert_eq!(password_credentials, &creds);
        } else {
            panic!("Status should be Active");
        }
        assert_eq!(user.updated_at(), &activate_time);
    }

    #[test]
    fn test_activate_from_deactivated() {
        let initial_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock = MockClock { now: initial_time };
        let mut user = setup_user(&clock);

        let creds = PasswordCredentials::new("phc".to_string(), &clock).unwrap();
        user.activate(creds.clone(), &clock).unwrap();
        user.deactivate("reason".to_string(), &clock).unwrap();

        let activate_time = initial_time + Duration::seconds(60);
        let clock2 = MockClock { now: activate_time };
        
        let result = user.activate(creds.clone(), &clock2);

        assert!(result.is_ok());
        assert!(matches!(user.status(), UserStatus::Active { .. }));
        assert_eq!(user.updated_at(), &activate_time);
    }

    #[test]
    fn test_activate_invalid_transition() {
        let clock = MockClock { now: Utc::now() };
        let mut user = setup_user(&clock);
        let creds = PasswordCredentials::new("phc".to_string(), &clock).unwrap();
        user.activate(creds.clone(), &clock).unwrap();

        // すでにActiveな状態で再度activate
        let result = user.activate(creds, &clock);
        assert!(matches!(result, Err(InvalidTransitionError)));
    }

    #[test]
    fn test_deactivate_success() {
        let initial_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock = MockClock { now: initial_time };
        let mut user = setup_user(&clock);
        let creds = PasswordCredentials::new("phc".to_string(), &clock).unwrap();
        user.activate(creds.clone(), &clock).unwrap();

        let deactivate_time = initial_time + Duration::seconds(40);
        let clock2 = MockClock { now: deactivate_time };
        let reason = "test reason".to_string();

        let result = user.deactivate(reason.clone(), &clock2);

        assert!(result.is_ok());
        if let UserStatus::Deactivated { password_credentials, deactivated_at, reason: r } = user.status() {
            assert_eq!(password_credentials, &creds);
            assert_eq!(deactivated_at, &deactivate_time);
            assert_eq!(r, &reason);
        } else {
            panic!("Status should be Deactivated");
        }
        assert_eq!(user.updated_at(), &deactivate_time);
    }

    #[test]
    fn test_deactivate_invalid_transition() {
        let clock = MockClock { now: Utc::now() };
        let mut user = setup_user(&clock);

        // Registered状態からdeactivateは不可
        let result = user.deactivate("reason".to_string(), &clock);
        assert!(matches!(result, Err(InvalidTransitionError)));
    }
}
