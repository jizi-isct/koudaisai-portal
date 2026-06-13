// TODO(sqlx移行): 削除された entities 型 (ApprovalRequest* / UserRead) を参照していたため
// Discord 通知メソッドをコメントアウトしてスタブ化。application 層へ再配線したら復元する。
// use crate::entities::approval_request::{
//     ApprovalRequestStatus, ApprovalRequestType, CreateApprovalRequest, ReadApprovalRequest,
// };
// use crate::entities::user::UserRead;
use aws_sdk_s3::config::http::HttpResponse;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use reqwest::Url;
use serenity::prelude::SerenityError;
use thiserror::Error;

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

    /* TODO(sqlx移行): 削除された entities 型に依存していたためコメントアウト。
    /// 承認申請が発行された旨の通知をDiscordに送信する
    pub async fn send_approval_request_issue_message(
        &self,
        base_url: &str,
        approval_request_id: &Uuid,
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
        approval_request_id: &Uuid,
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
    */
}
