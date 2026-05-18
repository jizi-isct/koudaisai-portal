use crate::entities::approval_request::{
    ApprovalRequestStatus, ApprovalRequestType, CreateApprovalRequest, ReadApprovalRequest,
};
use crate::entities::user::UserRead;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::http::HttpResponse;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use openidconnect::core::CoreUserInfoClaims;
use reqwest::Url;
use serenity::all::CreateAttachment;
use serenity::builder::{CreateEmbed, ExecuteWebhook};
use serenity::http::Http;
use serenity::model::webhook::Webhook;
use serenity::prelude::SerenityError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Discord {
    pub approval_request_url: String,
    pub approval_request_thread_id: Option<u64>,
}

#[derive(Error, Debug)]
pub enum SendApprovalRequestIssueMessageError {
    #[error(transparent)]
    SerenityError(#[from] SerenityError),
    #[error(transparent)]
    AwsByteStreamError(#[from] aws_smithy_types::byte_stream::error::Error),
    #[error(transparent)]
    S3Error(#[from] SdkError<GetObjectError, HttpResponse>),
}
impl Discord {
    pub fn new<T: Into<String>>(approval_request_url: T) -> Discord {
        let url_string = approval_request_url.into();
        let approval_request_thread_id = if let Ok(url) = Url::parse(&url_string) {
            url.query_pairs()
                .find(|(key, _)| key == "thread_id")
                .and_then(|(_, value)| value.parse::<u64>().ok())
        } else {
            None
        };

        Self {
            approval_request_url: url_string,
            approval_request_thread_id,
        }
    }

    /// 承認申請が発行された旨の通知をDiscordに送信する
    pub async fn send_approval_request_issue_message(
        &self,
        base_url: &str,
        approval_request_id: &uuid::Uuid,
        approval_request: &CreateApprovalRequest,
        issued_by: &UserRead,
        s3_client: &S3Client,
        s3_bucket: &str,
    ) -> Result<(), SendApprovalRequestIssueMessageError> {
        let http = Http::new(&self.approval_request_url);
        let webhook = Webhook::from_url(&http, &self.approval_request_url).await?;

        match &approval_request.r#type {
            ApprovalRequestType::TypeEditExhibitionInfo {
                description,
                icon_key,
            } => {
                let username = format!("{}の{}", issued_by.group_id, issued_by.name);
                let description = description
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("変更なし");
                let icon_text = match icon_key {
                    Some(_) => "変更あり",
                    None => "変更なし",
                };

                let mut builder = ExecuteWebhook::new()
                    .username(&username)
                    .embed(
                        CreateEmbed::new()
                            .title("企画内容訂正申請が出されました")
                            .description(&format!("[詳細を閲覧]({}/admin/approval_requests/review?approval_request_id={})", base_url, approval_request_id))
                            .field(
                                "申請事由",
                                &approval_request.issue_reason,
                                false,
                            )
                            .field(
                                "申請者",
                                &username,
                                false,
                            )
                            .field(
                                "企画内容紹介文",
                                description,
                                false,
                            )
                            .field(
                                "アイコン",
                                icon_text,
                                true,
                            )
                            .attachment(icon_text)
                            .color(0x0a9fd6)
                    );

                // Add thread_id using builder.in_thread method
                if let Some(thread_id) = self.approval_request_thread_id {
                    builder = builder.in_thread(thread_id);
                }

                // Download the icon from S3 if icon_key exists
                if let Some(key) = icon_key {
                    let response = s3_client
                        .get_object()
                        .bucket(s3_bucket)
                        .key(key)
                        .send()
                        .await?;
                    let bytes = response.body.collect().await?.into_bytes();
                    let attachment = CreateAttachment::bytes(bytes.to_vec(), key);
                    builder = builder.add_file(attachment);
                }

                webhook.execute(&http, false, builder).await?;
            }
        }
        Ok(())
    }

    /// 承認申請が承認/却下された旨の通知をDiscordに送信
    pub async fn send_approval_request_approval_message(
        &self,
        base_url: &str,
        approval_request_id: &uuid::Uuid,
        approval_request: &ReadApprovalRequest,
        issued_by: &UserRead,
        approved_by: &CoreUserInfoClaims,
    ) -> Result<(), SendApprovalRequestIssueMessageError> {
        let http = Http::new(&self.approval_request_url);
        let webhook = Webhook::from_url(&http, &self.approval_request_url).await?;

        match &approval_request.r#type {
            ApprovalRequestType::TypeEditExhibitionInfo { .. } => {
                let username = format!("{}の{}", issued_by.group_id, issued_by.name);

                // Determine approval status and color
                let (status_text, status_color) = match approval_request.status {
                    ApprovalRequestStatus::Approved => ("承認されました", 0x00ff00), // Green
                    ApprovalRequestStatus::Rejected => ("却下されました", 0xff0000), // Red
                    _ => ("処理されました", 0x0a9fd6),                               // Default blue
                };

                let approver_name = approved_by
                    .name()
                    .and_then(|n| n.get(None))
                    .map(|s| s.as_str())
                    .unwrap_or("管理者");

                let mut embed = CreateEmbed::new()
                    .title(&format!("企画内容訂正申請が{}", status_text))
                    .description(&format!(
                        "[詳細を閲覧]({}/admin/approval_requests/review?approval_request_id={})",
                        base_url, approval_request_id
                    ))
                    .field("申請者", &username, false)
                    .color(status_color);

                // Add approval reason if available
                if let Some(approval_reason) = &approval_request.approval_reason {
                    embed = embed.field("承認/却下理由", approval_reason, false);
                }

                let mut builder = ExecuteWebhook::new().username(approver_name).embed(embed);

                // Add thread_id using builder.in_thread method
                if let Some(thread_id) = self.approval_request_thread_id {
                    builder = builder.in_thread(thread_id);
                }

                webhook.execute(&http, false, builder).await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use serenity::builder::ExecuteWebhook;
    use serenity::http::Http;
    use serenity::prelude::SerenityError;
    use std::sync::{Arc, Mutex};

    #[async_trait]
    pub trait WebhookLike: Send + Sync {
        async fn execute(&self, http: &Http, wait: bool, builder: ExecuteWebhook) -> Result<Option<serenity::all::Message>, SerenityError>;
    }

    #[async_trait]
    impl WebhookLike for serenity::model::webhook::Webhook {
        async fn execute(&self, http: &Http, wait: bool, builder: ExecuteWebhook) -> Result<Option<serenity::all::Message>, SerenityError> {
            self.execute(http, wait, builder).await
        }
    }

    struct MockWebhook {
        called: Arc<Mutex<bool>>,
    }

    #[async_trait]
    impl WebhookLike for MockWebhook {
        async fn execute(&self, _http: &Http, _wait: bool, _builder: ExecuteWebhook) -> Result<Option<serenity::all::Message>, SerenityError> {
            let mut c = self.called.lock().unwrap();
            *c = true;
            Ok(None)
        }
    }

    // Test helper: same logic as production but accepts a WebhookLike and optional icon bytes
    async fn send_issue_with_webhook(
        d: &Discord,
        base_url: &str,
        approval_request_id: &uuid::Uuid,
        approval_request: &crate::entities::approval_request::CreateApprovalRequest,
        issued_by: &crate::entities::user::UserRead,
        webhook: &impl WebhookLike,
        icon_bytes: Option<Vec<u8>>,
        icon_key_name: Option<&str>,
    ) -> Result<(), SendApprovalRequestIssueMessageError> {
        match &approval_request.r#type {
            crate::entities::approval_request::ApprovalRequestType::TypeEditExhibitionInfo {
                description,
                icon_key,
            } => {
                let username = format!("{}の{}", issued_by.group_id, issued_by.name);
                let description = description
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("変更なし");
                let icon_text = match icon_key {
                    Some(_) => "変更あり",
                    None => "変更なし",
                };

                let mut builder = ExecuteWebhook::new()
                    .username(&username)
                    .embed(
                        CreateEmbed::new()
                            .title("企画内容訂正申請が出されました")
                            .description(&format!("[詳細を閲覧]({}/admin/approval_requests/review?approval_request_id={})", base_url, approval_request_id))
                            .field(
                                "申請事由",
                                &approval_request.issue_reason,
                                false,
                            )
                            .field(
                                "申請者",
                                &username,
                                false,
                            )
                            .field(
                                "企画内容紹介文",
                                description,
                                false,
                            )
                            .field(
                                "アイコン",
                                icon_text,
                                true,
                            )
                            .attachment(icon_text)
                            .color(0x0a9fd6)
                    );

                if let Some(thread_id) = d.approval_request_thread_id {
                    builder = builder.in_thread(thread_id);
                }

                if let Some(bytes) = icon_bytes {
                    let key = icon_key_name.unwrap_or("icon");
                    let attachment = CreateAttachment::bytes(bytes.to_vec(), key);
                    builder = builder.add_file(attachment);
                }

                webhook.execute(&Http::new(&d.approval_request_url), false, builder).await.map(|_| ())?;
            }
        }
        Ok(())
    }

    async fn send_approval_with_webhook(
        d: &Discord,
        base_url: &str,
        approval_request_id: &uuid::Uuid,
        approval_request: &crate::entities::approval_request::ReadApprovalRequest,
        issued_by: &crate::entities::user::UserRead,
        approved_by_name: Option<&str>,
        webhook: &impl WebhookLike,
    ) -> Result<(), SendApprovalRequestIssueMessageError> {
        match &approval_request.r#type {
            crate::entities::approval_request::ApprovalRequestType::TypeEditExhibitionInfo { .. } => {
                let username = format!("{}の{}", issued_by.group_id, issued_by.name);

                let (status_text, status_color) = match approval_request.status {
                    crate::entities::approval_request::ApprovalRequestStatus::Approved => ("承認されました", 0x00ff00),
                    crate::entities::approval_request::ApprovalRequestStatus::Rejected => ("却下されました", 0xff0000),
                    _ => ("処理されました", 0x0a9fd6),
                };

                let approver_name = approved_by_name.unwrap_or("管理者");

                let mut embed = CreateEmbed::new()
                    .title(&format!("企画内容訂正申請が{}", status_text))
                    .description(&format!(
                        "[詳細を閲覧]({}/admin/approval_requests/review?approval_request_id={})",
                        base_url, approval_request_id
                    ))
                    .field("申請者", &username, false)
                    .color(status_color);

                if let Some(approval_reason) = &approval_request.approval_reason {
                    embed = embed.field("承認/却下理由", approval_reason, false);
                }

                let mut builder = ExecuteWebhook::new().username(approver_name).embed(embed);

                if let Some(thread_id) = d.approval_request_thread_id {
                    builder = builder.in_thread(thread_id);
                }

                webhook.execute(&Http::new(&d.approval_request_url), false, builder).await.map(|_| ())?;
            }
        }
        Ok(())
    }

    #[test]
    fn discord_new_parses_thread_id() {
        let url = "https://example.com/webhook?thread_id=12345";
        let d = Discord::new(url);
        assert_eq!(d.approval_request_thread_id, Some(12345));
    }

    #[test]
    fn discord_new_no_thread_id() {
        let url = "https://example.com/webhook";
        let d = Discord::new(url);
        assert_eq!(d.approval_request_thread_id, None);
    }

    #[test]
    fn discord_new_invalid_url() {
        let url = "not a url";
        let d = Discord::new(url);
        assert_eq!(d.approval_request_thread_id, None);
    }

    #[tokio::test]
    async fn test_send_issue_with_mock_webhook_and_icon() {
        use crate::entities::approval_request::{ApprovalRequestType, CreateApprovalRequest};
        use crate::entities::user::UserRead;
        use chrono::Utc;
        use uuid::Uuid;

        let called = Arc::new(Mutex::new(false));
        let mock = MockWebhook { called: called.clone() };

        let d = Discord::new("https://example.com/webhook?thread_id=42");
        let approval_request_id = Uuid::new_v4();
        let req = CreateApprovalRequest {
            r#type: ApprovalRequestType::TypeEditExhibitionInfo { description: Some("desc".to_string()), icon_key: Some("k".to_string()) },
            issue_reason: "reason".to_string(),
        };

        let issued_by = UserRead {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: "GroupName".to_string(),
            m_address: "a@b.c".to_string(),
            group_id: "G-1".to_string(),
            password_updated_at: Utc::now(),
        };

        send_issue_with_webhook(&d, "https://base", &approval_request_id, &req, &issued_by, &mock, Some(vec![1,2,3]), Some("k")).await.unwrap();
        assert!(*called.lock().unwrap());
    }

    #[tokio::test]
    async fn test_send_approval_with_mock_webhook() {
        use crate::entities::approval_request::{ApprovalRequestType, ReadApprovalRequest, ApprovalRequestStatus};
        use crate::entities::user::UserRead;
        use chrono::Utc;
        use uuid::Uuid;

        let called = Arc::new(Mutex::new(false));
        let mock = MockWebhook { called: called.clone() };

        let d = Discord::new("https://example.com/webhook");
        let approval_request_id = Uuid::new_v4();
        let req = ReadApprovalRequest {
            id: Uuid::new_v4(),
            issued_at: Utc::now(),
            issued_by: Uuid::new_v4(),
            r#type: ApprovalRequestType::TypeEditExhibitionInfo { description: None, icon_key: None },
            status: ApprovalRequestStatus::Approved,
            approved_by: None,
            issue_reason: "reason".to_string(),
            approval_reason: Some("ok".to_string()),
        };

        let issued_by = UserRead {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: "GroupName".to_string(),
            m_address: "a@b.c".to_string(),
            group_id: "G-1".to_string(),
            password_updated_at: Utc::now(),
        };

        // Make a dummy CoreUserInfoClaims
        let approved_by_name: Option<&str> = None;

        send_approval_with_webhook(&d, "https://base", &approval_request_id, &req, &issued_by, approved_by_name, &mock).await.unwrap();
        assert!(*called.lock().unwrap());
    }
}
