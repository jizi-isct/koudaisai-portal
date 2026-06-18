use std::marker::PhantomData;

use crate::application::authz;
use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
use crate::application::ports::clock::Clock;
use crate::application::ports::discord::{Discord, DiscordEmbed, DiscordEmbedField, DiscordMessage};
use crate::application::ports::repositories::approval_request_repo::ApprovalRequestRepo;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::transaction::Transaction;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::admin_id::AdminId;
use crate::domain::approval_request::{
    ApprovalRequest, ApprovalRequestStatus, ApprovalRequestType,
};
use crate::domain::approval_request_id::ApprovalRequestId;
use crate::domain::user_id::UserId;

pub struct ApprovalRequestApp<
    'a,
    Tx: Transaction,
    AR: ApprovalRequestRepo<Tx>,
    MR: MembershipRepo<Tx>,
    C: Clock,
    D: Discord,
> {
    _phantom: PhantomData<&'a Tx>,
    approval_request_repo: &'a AR,
    membership_repo: &'a MR,
    clock: &'a C,
    discord: &'a D,
}

impl<
    'a,
    Tx: Transaction,
    AR: ApprovalRequestRepo<Tx>,
    MR: MembershipRepo<Tx>,
    C: Clock,
    D: Discord,
> ApprovalRequestApp<'a, Tx, AR, MR, C, D>
{
    pub fn new(
        approval_request_repo: &'a AR,
        membership_repo: &'a MR,
        clock: &'a C,
        discord: &'a D,
    ) -> Self {
        Self {
            _phantom: PhantomData,
            approval_request_repo,
            membership_repo,
            clock,
            discord,
        }
    }

    /// 申請者を表すラベル(所属グループ ID、例: `G-001`)を best-effort で求める。
    /// メンバーシップが取得できない場合は user_id 文字列にフォールバックする。
    async fn issuer_label(&self, user_id: UserId) -> String {
        match self.membership_repo.find_by_user_id(user_id).await {
            Ok(memberships) => memberships
                .first()
                .map(|m| m.group_id().to_string())
                .unwrap_or_else(|| user_id.to_string()),
            Err(_) => user_id.to_string(),
        }
    }

    /// 全ての承認申請を取得（管理者用）
    pub async fn get_all(
        &self,
        actor_ctx: &ActorContext,
    ) -> Result<Vec<ApprovalRequest>, ApplicationOperationError<FindError>> {
        if !authz::can_get_all_approval_requests(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }
        Ok(self.approval_request_repo.find_all().await?)
    }

    /// IDで承認申請を取得
    pub async fn get_by_id(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
    ) -> Result<Option<ApprovalRequest>, ApplicationOperationError<FindError>> {
        let Some(request) = self.approval_request_repo.find_by_id(id).await? else {
            return Ok(None);
        };

        let memberships_of_issuer = self
            .membership_repo
            .find_by_user_id(request.issued_by())
            .await?;

        if !authz::can_get_approval_request(actor_ctx, &request, &memberships_of_issuer) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        Ok(Some(request))
    }

    /// ユーザーが所属するグループのメンバーが発行した承認申請を取得
    pub async fn get_by_group_members(
        &self,
        actor_ctx: &ActorContext,
        user_id: UserId,
    ) -> Result<Vec<ApprovalRequest>, ApplicationOperationError<FindError>> {
        // 対象ユーザーのメンバーシップを取得
        let memberships_of_target = self.membership_repo.find_by_user_id(user_id).await?;
        if memberships_of_target.is_empty() {
            return Ok(vec![]);
        }

        // 対象ユーザーが所属するグループIDを取得（最初のグループを使用）
        let group_id = memberships_of_target[0].group_id();

        if !authz::can_get_group_approval_requests(actor_ctx, group_id) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        // グループの全メンバーのユーザーIDを取得
        let group_memberships = self.membership_repo.find_by_group_id(group_id).await?;
        let user_ids: Vec<UserId> = group_memberships.iter().map(|m| m.user_id()).collect();

        Ok(self
            .approval_request_repo
            .find_by_user_ids(&user_ids)
            .await?)
    }

    /// 承認申請を作成
    pub async fn create(
        &self,
        actor_ctx: &ActorContext,
        request_type: ApprovalRequestType,
        issue_reason: String,
    ) -> Result<ApprovalRequestId, ApplicationOperationError<InsertError>> {
        if !authz::can_create_approval_request(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let user_id = match actor_ctx {
            ActorContext::User { user_id, .. } => *user_id,
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let id = ApprovalRequestId::generate();
        let request = ApprovalRequest::create(id, user_id, request_type, issue_reason, self.clock)
            .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;

        self.approval_request_repo.insert(&request).await?;

        let issuer_label = self.issuer_label(request.issued_by()).await;
        let message = build_issue_message(&request, &issuer_label);
        self.discord
            .send(&message)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?;

        Ok(id)
    }

    /// 承認申請を承認
    pub async fn approve(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
        approval_reason: Option<String>,
    ) -> Result<ApprovalRequest, ApplicationOperationError<UpdateError>> {
        if !authz::can_approve_or_reject_approval_request(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let approver_id = match actor_ctx {
            ActorContext::Admin { user_id, .. } => AdminId::new((*user_id).into()),
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let mut request = self
            .approval_request_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
            .ok_or(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ))?;

        request
            .approve(approver_id, approval_reason, self.clock)
            .map_err(|_| {
                ApplicationOperationError::InvalidInput(
                    "Cannot approve non-pending request".to_string(),
                )
            })?;

        self.approval_request_repo.update(&request).await?;

        let issuer_label = self.issuer_label(request.issued_by()).await;
        let message = build_decision_message(&request, &issuer_label);
        self.discord
            .send(&message)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?;

        Ok(request)
    }

    /// 承認申請を却下
    pub async fn reject(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
        rejection_reason: Option<String>,
    ) -> Result<ApprovalRequest, ApplicationOperationError<UpdateError>> {
        if !authz::can_approve_or_reject_approval_request(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let rejector_id = match actor_ctx {
            ActorContext::Admin { user_id, .. } => AdminId::new((*user_id).into()),
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        let mut request = self
            .approval_request_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
            .ok_or(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ))?;

        request
            .reject(rejector_id, rejection_reason, self.clock)
            .map_err(|_| {
                ApplicationOperationError::InvalidInput(
                    "Cannot reject non-pending request".to_string(),
                )
            })?;

        self.approval_request_repo.update(&request).await?;

        let issuer_label = self.issuer_label(request.issued_by()).await;
        let message = build_decision_message(&request, &issuer_label);
        self.discord
            .send(&message)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?;

        Ok(request)
    }

    /// 承認申請をクローズ（申請者によるキャンセル）
    pub async fn close(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        let mut request = self
            .approval_request_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
            .ok_or(ApplicationOperationError::OperationFailed(
                UpdateError::NotFound,
            ))?;

        if !authz::can_close_approval_request(actor_ctx, &request) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        request.close(self.clock).map_err(|_| {
            ApplicationOperationError::InvalidInput("Cannot close non-pending request".to_string())
        })?;

        self.approval_request_repo.update(&request).await?;
        Ok(())
    }

    /// 承認申請を削除
    pub async fn delete(
        &self,
        actor_ctx: &ActorContext,
        id: ApprovalRequestId,
    ) -> Result<(), ApplicationOperationError<DeleteError>> {
        if !authz::can_delete_approval_request(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        self.approval_request_repo.delete(id).await?;
        Ok(())
    }
}

/// 承認申請発行時の Discord メッセージを組み立てる。
fn build_issue_message(request: &ApprovalRequest, issuer_label: &str) -> DiscordMessage {
    match request.request_type() {
        ApprovalRequestType::EditExhibitionInfo {
            description,
            icon_key,
        } => {
            let description_text = description.as_deref().unwrap_or("変更なし");
            let icon_text = if icon_key.is_some() {
                "変更あり"
            } else {
                "変更なし"
            };

            DiscordMessage {
                username: Some(issuer_label.to_string()),
                embeds: vec![DiscordEmbed {
                    title: Some("企画内容訂正申請が出されました".to_string()),
                    description: Some(review_link(request.id())),
                    color: Some(0x0a9fd6),
                    fields: vec![
                        DiscordEmbedField {
                            name: "申請事由".to_string(),
                            value: request.issue_reason().to_string(),
                            inline: false,
                        },
                        DiscordEmbedField {
                            name: "申請者".to_string(),
                            value: issuer_label.to_string(),
                            inline: false,
                        },
                        DiscordEmbedField {
                            name: "企画内容紹介文".to_string(),
                            value: description_text.to_string(),
                            inline: false,
                        },
                        DiscordEmbedField {
                            name: "アイコン".to_string(),
                            value: icon_text.to_string(),
                            inline: true,
                        },
                    ],
                }],
                attachments: vec![],
            }
        }
    }
}

/// 承認/却下時の Discord メッセージを組み立てる。
fn build_decision_message(request: &ApprovalRequest, issuer_label: &str) -> DiscordMessage {
    let (status_text, color, reason) = match request.status() {
        ApprovalRequestStatus::Approved {
            approval_reason, ..
        } => ("承認されました", 0x00ff00, approval_reason.clone()),
        ApprovalRequestStatus::Rejected {
            rejection_reason, ..
        } => ("却下されました", 0xff0000, rejection_reason.clone()),
        _ => ("処理されました", 0x0a9fd6, None),
    };

    let mut fields = vec![DiscordEmbedField {
        name: "申請者".to_string(),
        value: issuer_label.to_string(),
        inline: false,
    }];
    if let Some(reason) = reason {
        fields.push(DiscordEmbedField {
            name: "承認/却下理由".to_string(),
            value: reason,
            inline: false,
        });
    }

    DiscordMessage {
        username: Some("管理者".to_string()),
        embeds: vec![DiscordEmbed {
            title: Some(format!("企画内容訂正申請が{}", status_text)),
            description: Some(review_link(request.id())),
            color: Some(color),
            fields,
        }],
        attachments: vec![],
    }
}

/// 管理画面のレビューページへのリンク文字列(Markdown)。
/// `base_url` は application 層に存在しないため相対パスで表現する(本番配線時に補完する)。
fn review_link(id: ApprovalRequestId) -> String {
    format!(
        "[詳細を閲覧](/admin/approval_requests/review?approval_request_id={})",
        id
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::group::GroupType;
    use crate::infra::memory::approval_request_repo_impl::MemoryApprovalRequestRepo;
    use crate::infra::memory::clock_impl::MemoryClock;
    use crate::infra::memory::discord_impl::MemoryDiscord;
    use crate::infra::memory::membership_repo_impl::MemoryMembershipRepo;
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;

    fn user_ctx() -> ActorContext {
        ActorContext::User {
            user_id: UserId::new(Uuid::new_v4()),
            memberships: vec![],
            group_type: GroupType::GeneralProject,
        }
    }

    fn admin_ctx() -> ActorContext {
        ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec!["koudaisai-portal:admin:approval-request:approve".to_string()],
        }
    }

    #[tokio::test]
    async fn create_sends_issue_notification() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let app = ApprovalRequestApp::new(&ar, &mr, &clock, &discord);

        app.create(
            &user_ctx(),
            ApprovalRequestType::EditExhibitionInfo {
                description: Some("新しい紹介文".to_string()),
                icon_key: None,
            },
            "理由".to_string(),
        )
        .await
        .expect("create should succeed");

        let messages = discord.sent_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(
            messages[0].embeds[0].title.as_deref(),
            Some("企画内容訂正申請が出されました")
        );
    }

    #[tokio::test]
    async fn approve_sends_decision_notification_with_reason() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let app = ApprovalRequestApp::new(&ar, &mr, &clock, &discord);

        let id = app
            .create(
                &user_ctx(),
                ApprovalRequestType::EditExhibitionInfo {
                    description: None,
                    icon_key: None,
                },
                "理由".to_string(),
            )
            .await
            .expect("create should succeed");

        app.approve(&admin_ctx(), id, Some("問題ありません".to_string()))
            .await
            .expect("approve should succeed");

        // 作成時(1通目)と承認時(2通目)の 2 通が送られている。
        let messages = discord.sent_messages();
        assert_eq!(messages.len(), 2);
        assert_eq!(
            messages[1].embeds[0].title.as_deref(),
            Some("企画内容訂正申請が承認されました")
        );
        assert!(
            messages[1].embeds[0]
                .fields
                .iter()
                .any(|f| f.name == "承認/却下理由" && f.value == "問題ありません")
        );
    }
}
