use crate::domain::email_address::EmailAddress;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait Email {
    async fn send(&self, address: &EmailAddress, body: &str) -> Result<(), DomainError>;
}