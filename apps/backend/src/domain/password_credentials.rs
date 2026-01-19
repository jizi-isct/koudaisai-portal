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
