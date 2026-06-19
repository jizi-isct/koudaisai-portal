use crate::application::error::ApplicationError;
use crate::domain::email_address::EmailAddress;

#[async_trait::async_trait]
pub trait Email {
    /// メールを送信する。`subject` は件名、`body` は本文(text/plain)。
    async fn send(
        &self,
        address: &EmailAddress,
        subject: &str,
        body: &str,
    ) -> Result<(), ApplicationError>;
}
