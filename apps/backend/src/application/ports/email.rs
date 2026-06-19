use crate::application::error::ApplicationError;
use crate::domain::email_address::EmailAddress;

#[async_trait::async_trait]
pub trait Email {
    async fn send(&self, address: &EmailAddress, body: &str) -> Result<(), ApplicationError>;
}
