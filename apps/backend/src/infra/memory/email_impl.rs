use std::sync::{Arc, RwLock};
use anyhow::anyhow;
use async_trait::async_trait;
use crate::application::error::ApplicationError;
use crate::application::ports::email::Email;
use crate::domain::email_address::EmailAddress;
use crate::domain::error::DomainError;

pub struct MemoryEmail {
    sent_emails: Arc<RwLock<Vec<(EmailAddress, String)>>>,
}

impl MemoryEmail {
    pub fn new() -> Self {
        Self {
            sent_emails: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn sent_emails(&self) -> Vec<(EmailAddress, String)> {
        self.sent_emails.read().unwrap().clone()
    }
}

#[async_trait]
impl Email for MemoryEmail {
    async fn send(&self, address: &EmailAddress, body: &str) -> Result<(), ApplicationError> {
        let mut sent_emails = self.sent_emails.write().map_err(|e| ApplicationError::InternalError(anyhow!(e.to_string())))?;
        sent_emails.push((address.clone(), body.to_string()));
        Ok(())
    }
}
