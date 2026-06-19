//! 本番用の [`Email`] 実装(SendGrid v3)。
//!
//! 件名・本文(text/plain)を 1 通送る最小実装。送信は best-effort で，
//! 失敗は呼び出し側(HTTP 層)がリクエストパス外で握りつぶす想定。

use crate::application::error::ApplicationError;
use crate::application::ports::email::Email;
use crate::domain::email_address::EmailAddress;
use anyhow::anyhow;
use async_trait::async_trait;
use sendgrid::v3::{Content, Email as SgEmail, Message, Personalization, Sender};

pub struct SendgridEmail {
    sender: Sender,
    from_address: String,
}

impl SendgridEmail {
    pub fn new(api_key: String, from_address: String) -> Self {
        Self {
            sender: Sender::new(api_key, None),
            from_address,
        }
    }

    /// SendGrid 設定(API キー・送信元アドレス)から構築する。
    pub fn from_config(cfg: &crate::config::Sendgrid) -> Self {
        Self::new(cfg.api_key.clone(), cfg.sender_address.clone())
    }
}

#[async_trait]
impl Email for SendgridEmail {
    async fn send(
        &self,
        address: &EmailAddress,
        subject: &str,
        body: &str,
    ) -> Result<(), ApplicationError> {
        let message = Message::new(SgEmail::new(self.from_address.clone()))
            .set_subject(subject)
            .add_content(
                Content::new()
                    .set_content_type("text/plain")
                    .set_value(body),
            )
            .add_personalization(Personalization::new(SgEmail::new(address.address.clone())));

        let response = self
            .sender
            .send(&message)
            .await
            .map_err(|e| ApplicationError::InternalError(anyhow!("sendgrid send failed: {e}")))?;

        if !response.status().is_success() {
            return Err(ApplicationError::InternalError(anyhow!(
                "sendgrid returned status {}",
                response.status()
            )));
        }
        Ok(())
    }
}
