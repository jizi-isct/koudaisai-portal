use crate::domain::error::{DomainError, FactoryError};
use chrono::{DateTime, Utc};
use crate::application::ports::clock::Clock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PasswordCredentials {
    phc: String,
    changed_at: DateTime<Utc>,
}

impl PasswordCredentials {
    pub fn new<C: Clock>(phc: String, clock: C) -> Result<Self, FactoryError> {
        Ok(Self {
            phc,
            changed_at: clock.now(),
        })
    }

    pub fn restore(phc: &str, changed_at: DateTime<Utc>) -> Result<Self, FactoryError> {
        Ok(Self {
            phc: phc.to_string(),
            changed_at,
        })
    }

    pub fn phc(&self) -> &str {
        &self.phc
    }

    pub fn changed_at(&self) -> &DateTime<Utc> {
        &self.changed_at
    }

    pub fn change<C: Clock>(&mut self, phc: String, clock: C) -> Result<(), DomainError> {
        self.phc = phc;
        self.changed_at = clock.now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::clock::Clock;
    use chrono::{TimeZone, Utc};

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    #[test]
    fn test_new_success() {
        let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock = MockClock { now };
        let phc = "argon2id$v=19$m=4096,t=3,p=1$somesalt$somehash".to_string();

        let creds = PasswordCredentials::new(phc.clone(), clock).unwrap();

        assert_eq!(creds.phc(), phc);
        assert_eq!(creds.changed_at(), &now);
    }

    #[test]
    fn test_restore_success() {
        let phc = "argon2id$v=19$m=4096,t=3,p=1$somesalt$somehash";
        let changed_at = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();

        let creds = PasswordCredentials::restore(phc, changed_at).unwrap();

        assert_eq!(creds.phc(), phc);
        assert_eq!(creds.changed_at(), &changed_at);
    }

    #[test]
    fn test_change_success() {
        let initial_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock1 = MockClock { now: initial_time };
        let phc1 = "old_phc".to_string();
        let mut creds = PasswordCredentials::new(phc1, clock1).unwrap();

        let update_time = Utc.with_ymd_and_hms(2024, 1, 1, 1, 0, 0).unwrap();
        let clock2 = MockClock { now: update_time };
        let phc2 = "new_phc".to_string();

        let result = creds.change(phc2.clone(), clock2);

        assert!(result.is_ok());
        assert_eq!(creds.phc(), phc2);
        assert_eq!(creds.changed_at(), &update_time);
    }
}
