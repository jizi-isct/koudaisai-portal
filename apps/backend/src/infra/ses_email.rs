//! 本番用の [`Email`] 実装(Amazon SES v2)。
//!
//! 件名・本文(text/plain, UTF-8)を 1 通送る最小実装。送信は best-effort で，
//! 失敗は呼び出し側(HTTP 層)がリクエストパス外で握りつぶす想定。

use crate::application::error::ApplicationError;
use crate::application::ports::email::Email;
use crate::domain::email_address::EmailAddress;
use anyhow::anyhow;
use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_sesv2::config::Credentials;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};

pub struct SesEmail {
    client: aws_sdk_sesv2::Client,
    from_address: String,
}

impl SesEmail {
    pub fn new(client: aws_sdk_sesv2::Client, from_address: String) -> Self {
        Self {
            client,
            from_address,
        }
    }

    /// SES 設定(リージョン・認証情報・送信元アドレス)から aws-sdk-sesv2 クライアントを構築する。
    pub async fn from_config(cfg: &crate::config::Ses) -> Self {
        let shared_cfg = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(
                Credentials::builder()
                    .access_key_id(cfg.access_key_id.clone())
                    .secret_access_key(cfg.secret_access_key.clone())
                    .provider_name("default-provider")
                    .build(),
            )
            .region(aws_config::Region::new(cfg.region.clone()))
            .load()
            .await;

        Self::new(
            aws_sdk_sesv2::Client::new(&shared_cfg),
            cfg.sender_address.clone(),
        )
    }
}

#[async_trait]
impl Email for SesEmail {
    async fn send(
        &self,
        address: &EmailAddress,
        subject: &str,
        body: &str,
    ) -> Result<(), ApplicationError> {
        let to_internal =
            |e: aws_sdk_sesv2::error::BuildError| ApplicationError::InternalError(anyhow!(e));

        let subject_content = Content::builder()
            .data(subject)
            .charset("UTF-8")
            .build()
            .map_err(to_internal)?;
        let body_content = Content::builder()
            .data(body)
            .charset("UTF-8")
            .build()
            .map_err(to_internal)?;

        let message = Message::builder()
            .subject(subject_content)
            .body(Body::builder().text(body_content).build())
            .build();

        let destination = Destination::builder()
            .to_addresses(address.address.clone())
            .build();

        self.client
            .send_email()
            .from_email_address(self.from_address.clone())
            .destination(destination)
            .content(EmailContent::builder().simple(message).build())
            .send()
            .await
            .map_err(|e| {
                ApplicationError::InternalError(anyhow!(
                    "ses send failed: {}",
                    aws_sdk_sesv2::error::DisplayErrorContext(&e)
                ))
            })?;

        Ok(())
    }
}
