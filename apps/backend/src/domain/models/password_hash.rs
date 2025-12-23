use crate::domain::models::error::DomainError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(phc: String) -> Result<Self, DomainError> {
        Ok(Self(phc))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}