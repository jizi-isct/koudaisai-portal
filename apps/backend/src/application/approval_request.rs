use std::marker::PhantomData;

use crate::application::authz;
use crate::application::error::{
    ApplicationOperationError, DeleteError, FindError, InsertError, UpdateError,
};
use crate::application::ports::clock::Clock;
use crate::application::ports::discord::{
    Discord, DiscordAttachment, DiscordEmbed, DiscordEmbedField, DiscordMessage,
};
use crate::application::ports::events26_api::{Events26Api, UpdateIconError};
use crate::application::ports::object_storage::ObjectStorage;
use crate::application::ports::repositories::approval_request_repo::ApprovalRequestRepo;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::ports::repositories::notification_repo::NotificationRepo;
use crate::application::ports::repositories::settings_repo::SettingsRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::transaction::Transaction;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::admin_id::AdminId;
use crate::domain::approval_request::{
    ApprovalRequest, ApprovalRequestStatus, ApprovalRequestType,
};
use crate::domain::approval_request_id::ApprovalRequestId;
use crate::domain::group_id::GroupId;
use crate::domain::notification::{Notification, NotificationType};
use crate::domain::notification_id::NotificationId;
use crate::domain::target_specifier::TargetSpecifier;
use crate::domain::user_id::UserId;

pub struct ApprovalRequestApp<
    'a,
    Tx: Transaction,
    AR: ApprovalRequestRepo<Tx>,
    MR: MembershipRepo<Tx>,
    UR: UserRepo<Tx>,
    C: Clock,
    D: Discord,
    OS: ObjectStorage,
    EA: Events26Api,
    NR: NotificationRepo<Tx>,
    STR: SettingsRepo,
> {
    _phantom: PhantomData<&'a Tx>,
    approval_request_repo: &'a AR,
    membership_repo: &'a MR,
    user_repo: &'a UR,
    clock: &'a C,
    discord: &'a D,
    object_storage: &'a OS,
    /// 企画情報API(events26)。承認した編集内容を企画へ反映するために使う。
    events26_api: &'a EA,
    /// 承認/却下の結果をポータル上の通知として残すために使う。
    notification_repo: &'a NR,
    settings_repo: &'a STR,
    base_url: &'a str,
}

impl<
    'a,
    Tx: Transaction,
    AR: ApprovalRequestRepo<Tx>,
    MR: MembershipRepo<Tx>,
    UR: UserRepo<Tx>,
    C: Clock,
    D: Discord,
    OS: ObjectStorage,
    EA: Events26Api,
    NR: NotificationRepo<Tx>,
    STR: SettingsRepo,
> ApprovalRequestApp<'a, Tx, AR, MR, UR, C, D, OS, EA, NR, STR>
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        approval_request_repo: &'a AR,
        membership_repo: &'a MR,
        user_repo: &'a UR,
        clock: &'a C,
        discord: &'a D,
        object_storage: &'a OS,
        events26_api: &'a EA,
        notification_repo: &'a NR,
        settings_repo: &'a STR,
        base_url: &'a str,
    ) -> Self {
        Self {
            _phantom: PhantomData,
            approval_request_repo,
            membership_repo,
            user_repo,
            clock,
            discord,
            object_storage,
            events26_api,
            notification_repo,
            settings_repo,
            base_url,
        }
    }

    /// 申請者ラベル `{group_id}の{name}` を best-effort で求める。
    /// 承認/却下では申請者が操作者と別人のため、グループは membership_repo、
    /// 氏名は user_repo から解決する。取得できない要素は省く。
    async fn issuer_label_by_id(&self, user_id: UserId) -> String {
        let group = match self.membership_repo.find_by_user_id(user_id).await {
            Ok(memberships) => memberships.first().map(|m| m.group_id().to_string()),
            Err(_) => None,
        };
        let name = match self.user_repo.find_by_id(user_id).await {
            Ok(Some(user)) => Some(user.name().to_string()),
            _ => None,
        };
        match (group, name) {
            (Some(group), Some(name)) => format!("{group}の{name}"),
            (Some(group), None) => group,
            (None, Some(name)) => name,
            (None, None) => user_id.to_string(),
        }
    }

    /// 承認された申請の内容を events26 の企画へ反映する。
    ///
    /// 企画は申請の対象団体そのもの(団体 ID = 企画番号)。申請者は複数の団体に
    /// 所属しうるので、申請者からではなく申請が持つ団体から反映先を決める。
    ///
    /// 紹介文とアイコンはそれぞれ指定があるものだけを送る。企画を丸ごと
    /// 置き換えるとタグや開催予定まで巻き込むため、紹介文は専用の
    /// [`Events26Api::update_project_description`] を使う。
    async fn apply_to_events26(
        &self,
        request: &ApprovalRequest,
    ) -> Result<(), ApplicationOperationError<UpdateError>> {
        let ApprovalRequestType::EditExhibitionInfo {
            description,
            icon_key,
        } = request.request_type();

        if description.is_none() && icon_key.is_none() {
            return Ok(());
        }

        let project_id = request.group_id().to_string();

        if let Some(description) = description {
            self.events26_api
                .update_project_description(&project_id, description)
                .await?;
        }

        if let Some(key) = icon_key {
            let image = self
                .object_storage
                .get_object(key)
                .await
                .map_err(|e| ApplicationOperationError::InternalError(e.into()))?;
            let content_type = icon_content_type(key, &image).ok_or_else(|| {
                ApplicationOperationError::InvalidInput(format!(
                    "Cannot determine the icon format \
                     (supported: png, jpeg, gif, webp, heic): {key}"
                ))
            })?;
            self.events26_api
                .update_project_icon(&project_id, content_type, image)
                .await
                .map_err(|e| match e {
                    UpdateIconError::NotFound => {
                        ApplicationOperationError::OperationFailed(UpdateError::NotFound)
                    }
                    UpdateIconError::InvalidImage(reason) => {
                        ApplicationOperationError::InvalidInput(reason)
                    }
                    UpdateIconError::InternalError(error) => {
                        ApplicationOperationError::InternalError(error)
                    }
                })?;
        }

        Ok(())
    }

    /// 承認/却下の結果をポータル上の通知([`NotificationType::ApprovalRequest`])として発行する。
    ///
    /// 宛先は申請者本人と申請の対象団体(同じ団体の他の責任者にも見えるように)。
    /// 申請者が所属する他の団体はこの申請と関係がないので含めない。
    /// 申請の状態は既に保存済みなので、通知の発行に失敗しても承認/却下自体は
    /// 成立させ、ログに残すだけにする(Discord 通知と同じ best-effort)。
    async fn notify_decision(&self, request: &ApprovalRequest, decided_by: AdminId) {
        let targets = vec![
            TargetSpecifier::UserId(request.issued_by()),
            TargetSpecifier::GroupId(request.group_id()),
        ];

        let notification = match Notification::create(
            NotificationId::generate(),
            targets,
            NotificationType::approval_request(request.id()),
            Some(decided_by),
            self.clock,
        ) {
            Ok(notification) => notification,
            Err(error) => {
                tracing::error!(%error, "承認申請通知の作成に失敗しました");
                return;
            }
        };

        if let Err(error) = self.notification_repo.insert(&notification).await {
            tracing::error!(%error, "承認申請通知の保存に失敗しました");
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
    ///
    /// 対象団体は申請者が明示する。1 人が複数の団体に所属しうる仕様なので、
    /// 申請者から一意には決まらないため。自分が所属していない団体は指定できない。
    pub async fn create(
        &self,
        actor_ctx: &ActorContext,
        group_id: GroupId,
        request_type: ApprovalRequestType,
        issue_reason: String,
    ) -> Result<ApprovalRequestId, ApplicationOperationError<InsertError>> {
        if !authz::can_create_approval_request(actor_ctx) {
            return Err(ApplicationOperationError::Unauthorized);
        }

        if matches!(request_type, ApprovalRequestType::EditExhibitionInfo { .. })
            && !self
                .settings_repo
                .get()
                .await
                .map_err(|e| ApplicationOperationError::InternalError(e.into()))?
                .accept_correction_requests()
        {
            return Err(ApplicationOperationError::InvalidInput(
                "訂正申請の受付は終了しています".to_string(),
            ));
        }

        let (user_id, memberships) = match actor_ctx {
            ActorContext::User {
                user_id,
                memberships,
                ..
            } => (*user_id, memberships),
            _ => return Err(ApplicationOperationError::Unauthorized),
        };

        if !memberships
            .iter()
            .any(|membership| membership.group_id() == group_id)
        {
            return Err(ApplicationOperationError::Unauthorized);
        }

        let id = ApprovalRequestId::generate();
        let request = ApprovalRequest::create(
            id,
            user_id,
            group_id,
            request_type,
            issue_reason,
            self.clock,
        )
        .map_err(|e| ApplicationOperationError::InvalidInput(e.to_string()))?;

        self.approval_request_repo.insert(&request).await?;

        // 作成では操作者＝申請者なので ActorContext から氏名・グループを得る。
        let issuer_label = issuer_label_from_actor(actor_ctx);
        let mut message = build_issue_message(self.base_url, &request, &issuer_label);

        // アイコンが指定されていれば本体を取得して添付する(best-effort)。
        if let ApprovalRequestType::EditExhibitionInfo {
            icon_key: Some(key),
            ..
        } = request.request_type()
        {
            match self.object_storage.get_object(key).await {
                Ok(bytes) => message.attachments.push(DiscordAttachment {
                    file_name: key.clone(),
                    bytes,
                }),
                Err(error) => {
                    tracing::error!(%error, "承認申請アイコンの取得に失敗しました(添付をスキップ)")
                }
            }
        }

        // 通知は best-effort: 送信失敗で作成自体は失敗させず、ログに残すのみ。
        if let Err(error) = self.discord.send(&message).await {
            tracing::error!(%error, "承認申請(発行)の Discord 通知送信に失敗しました");
        }

        Ok(id)
    }

    /// 承認申請を承認し、申請内容を events26 の企画へ反映する。
    ///
    /// 反映は承認済みとして保存する**前**に行う。失敗したらエラーを返して申請を
    /// 審査中のまま残し、「承認したのに反映されていない」状態を作らない。
    /// Discord 通知だけは従来どおり best-effort。
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

        // 状態遷移の検査を先に済ませる(審査中でない申請を反映してしまわないため)。
        request
            .approve(approver_id, approval_reason, self.clock)
            .map_err(|_| {
                ApplicationOperationError::InvalidInput(
                    "Cannot approve non-pending request".to_string(),
                )
            })?;

        self.apply_to_events26(&request).await?;

        self.approval_request_repo.update(&request).await?;

        self.notify_decision(&request, approver_id).await;

        let issuer_label = self.issuer_label_by_id(request.issued_by()).await;
        let message = build_decision_message(
            self.base_url,
            &request,
            &issuer_label,
            &approver_name(actor_ctx),
        );
        // 通知は best-effort: 送信失敗で承認自体は失敗させず、ログに残すのみ。
        if let Err(error) = self.discord.send(&message).await {
            tracing::error!(%error, "承認申請(承認)の Discord 通知送信に失敗しました");
        }

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

        self.notify_decision(&request, rejector_id).await;

        let issuer_label = self.issuer_label_by_id(request.issued_by()).await;
        let message = build_decision_message(
            self.base_url,
            &request,
            &issuer_label,
            &approver_name(actor_ctx),
        );
        // 通知は best-effort: 送信失敗で却下自体は失敗させず、ログに残すのみ。
        if let Err(error) = self.discord.send(&message).await {
            tracing::error!(%error, "承認申請(却下)の Discord 通知送信に失敗しました");
        }

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

/// 操作者(申請者)の ActorContext から申請者ラベル `{group_id}の{name}` を作る。
/// 作成フロー専用(操作者＝申請者)。グループが無ければ氏名のみ。
fn issuer_label_from_actor(actor_ctx: &ActorContext) -> String {
    match actor_ctx {
        ActorContext::User {
            name, memberships, ..
        } => match memberships.first() {
            Some(m) => format!("{}の{}", m.group_id(), name),
            None => name.clone(),
        },
        _ => "不明な申請者".to_string(),
    }
}

/// アイコンのメディアタイプを判別する。events26 は `Content-Type` で形式を見るため、
/// 中身から決める必要がある。
///
/// オブジェクトストレージのキーは申請時に呼び出し側が決めており拡張子がある保証は
/// ないので、まずマジックバイトで判定し、それで決まらないときだけ拡張子に頼る。
/// events26 が受け付けない形式は `None`(承認を通さない)。
fn icon_content_type(key: &str, image: &[u8]) -> Option<&'static str> {
    let by_magic = if image.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("image/png")
    } else if image.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some("image/jpeg")
    } else if image.starts_with(b"GIF87a") || image.starts_with(b"GIF89a") {
        Some("image/gif")
    } else if image.len() >= 12 && image.starts_with(b"RIFF") && &image[8..12] == b"WEBP" {
        Some("image/webp")
    } else if image.len() >= 12
        && &image[4..8] == b"ftyp"
        && matches!(
            &image[8..12],
            b"heic" | b"heix" | b"hevc" | b"heim" | b"heis" | b"mif1" | b"msf1"
        )
    {
        Some("image/heic")
    } else {
        None
    };
    if by_magic.is_some() {
        return by_magic;
    }

    let extension = key.rsplit_once('.')?.1.to_ascii_lowercase();
    match extension.as_str() {
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        "webp" => Some("image/webp"),
        "heic" | "heif" => Some("image/heic"),
        _ => None,
    }
}

/// 承認/却下の操作者(管理者)の表示名。Admin 以外は "管理者" にフォールバック。
fn approver_name(actor_ctx: &ActorContext) -> String {
    match actor_ctx {
        ActorContext::Admin { name, .. } => name.clone(),
        _ => "管理者".to_string(),
    }
}

/// 承認申請発行時の Discord メッセージを組み立てる。
fn build_issue_message(
    base_url: &str,
    request: &ApprovalRequest,
    issuer_label: &str,
) -> DiscordMessage {
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
                    description: Some(review_link(base_url, request.id())),
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
fn build_decision_message(
    base_url: &str,
    request: &ApprovalRequest,
    issuer_label: &str,
    approver_name: &str,
) -> DiscordMessage {
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
        username: Some(approver_name.to_string()),
        embeds: vec![DiscordEmbed {
            title: Some(format!("企画内容訂正申請が{}", status_text)),
            description: Some(review_link(base_url, request.id())),
            color: Some(color),
            fields,
        }],
        attachments: vec![],
    }
}

/// 管理画面のレビューページへの絶対リンク文字列(Markdown)。
fn review_link(base_url: &str, id: ApprovalRequestId) -> String {
    format!(
        "[詳細を閲覧]({}/approval_requests/review?approval_request_id={})",
        base_url.trim_end_matches('/'),
        id
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::repositories::settings_repo::SettingsRepo;
    use crate::domain::email_address::EmailAddress;
    use crate::domain::group::GroupType;
    use crate::domain::group_id::GroupId;
    use crate::domain::membership::{Membership, Role};
    use crate::domain::user::User;
    use crate::infra::memory::approval_request_repo_impl::MemoryApprovalRequestRepo;
    use crate::infra::memory::clock_impl::MemoryClock;
    use crate::infra::memory::discord_impl::MemoryDiscord;
    use crate::infra::memory::events26_api_impl::MemoryEvents26Api;
    use crate::infra::memory::membership_repo_impl::MemoryMembershipRepo;
    use crate::infra::memory::notification_repo_impl::MemoryNotificationRepo;
    use crate::infra::memory::object_storage_impl::MemoryObjectStorage;
    use crate::infra::memory::settings_repo_impl::MemorySettingsRepo;
    use crate::infra::memory::user_repo_impl::MemoryUserRepo;
    use chrono::{TimeZone, Utc};
    use std::str::FromStr;
    use uuid::Uuid;

    const BASE_URL: &str = "https://portal.koudaisai.jp";

    /// 申請者役の `ActorContext`。対象団体は申請時に指定するので、所属も持たせる。
    fn user_ctx() -> ActorContext {
        let user_id = UserId::new(Uuid::new_v4());
        ActorContext::User {
            user_id,
            name: "山田太郎".to_string(),
            memberships: vec![Membership::new(
                group_id(),
                user_id,
                Role::FirstResponsible,
                &MemoryClock::new(Utc.timestamp_opt(0, 0).unwrap()),
            )],
            group_type: GroupType::GeneralProject,
        }
    }

    /// テストで使う対象団体。
    fn group_id() -> GroupId {
        GroupId::from_str("I-100").unwrap()
    }

    fn admin_ctx() -> ActorContext {
        ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            name: "承認担当".to_string(),
            claims: vec!["koudaisai-portal:admin:approval-request:approve".to_string()],
        }
    }

    #[tokio::test]
    async fn create_sends_issue_notification() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let ur = MemoryUserRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let os = MemoryObjectStorage::new();
        let e26 = MemoryEvents26Api::new();
        let nr = MemoryNotificationRepo::new();
        let settings = MemorySettingsRepo::new();
        let app = ApprovalRequestApp::new(
            &ar, &mr, &ur, &clock, &discord, &os, &e26, &nr, &settings, BASE_URL,
        );

        app.create(
            &user_ctx(),
            group_id(),
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
        // 申請者名(ActorContext の name)が送信者名に、団体付きのラベルが
        // 申請者フィールドに反映される。
        assert_eq!(messages[0].username.as_deref(), Some("I-100の山田太郎"));
        assert!(
            messages[0].embeds[0]
                .fields
                .iter()
                .any(|f| f.name == "申請者" && f.value == "I-100の山田太郎")
        );
        // 詳細リンクが base_url 由来の絶対 URL になっている。
        assert!(
            messages[0].embeds[0]
                .description
                .as_deref()
                .unwrap()
                .contains("https://admin.koudaisai.jp/approval_requests/review")
        );
    }

    #[tokio::test]
    async fn create_edit_exhibition_info_fails_when_correction_requests_are_closed() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let ur = MemoryUserRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let os = MemoryObjectStorage::new();
        let e26 = MemoryEvents26Api::new();
        let nr = MemoryNotificationRepo::new();
        let settings = MemorySettingsRepo::new();
        let mut current_settings = settings.get().await.unwrap();
        current_settings.change_accept_correction_requests(false);
        settings.save(&current_settings).await.unwrap();
        let app = ApprovalRequestApp::new(
            &ar, &mr, &ur, &clock, &discord, &os, &e26, &nr, &settings, BASE_URL,
        );

        let result = app
            .create(
                &user_ctx(),
                group_id(),
                ApprovalRequestType::EditExhibitionInfo {
                    description: Some("新しい紹介文".to_string()),
                    icon_key: None,
                },
                "理由".to_string(),
            )
            .await;

        assert!(matches!(
            result,
            Err(ApplicationOperationError::InvalidInput(_))
        ));
        assert!(discord.sent_messages().is_empty());
    }

    #[tokio::test]
    async fn approve_sends_decision_notification_with_reason() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let ur = MemoryUserRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let os = MemoryObjectStorage::new();
        let e26 = MemoryEvents26Api::new();
        let nr = MemoryNotificationRepo::new();
        let settings = MemorySettingsRepo::new();
        let app = ApprovalRequestApp::new(
            &ar, &mr, &ur, &clock, &discord, &os, &e26, &nr, &settings, BASE_URL,
        );

        // 承認通知の申請者名は user_repo から解決されるので、申請者を登録しておく。
        let (_issuer_id, issuer_group, issuer_ctx) = setup_issuer(&ur, &mr, now, "I-126").await;
        let id = app
            .create(
                &issuer_ctx,
                issuer_group,
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
        let decision = &messages[1];
        assert_eq!(
            decision.embeds[0].title.as_deref(),
            Some("企画内容訂正申請が承認されました")
        );
        // 承認者名(Admin の name)が送信者名に反映される。
        assert_eq!(decision.username.as_deref(), Some("承認担当"));
        // 申請者名(user_repo から解決)が申請者フィールドに反映される。
        assert!(
            decision.embeds[0]
                .fields
                .iter()
                .any(|f| f.name == "申請者" && f.value.contains("申請花子"))
        );
        assert!(
            decision.embeds[0]
                .fields
                .iter()
                .any(|f| f.name == "承認/却下理由" && f.value == "問題ありません")
        );
    }

    #[tokio::test]
    async fn create_with_icon_attaches_it() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let ur = MemoryUserRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let os = MemoryObjectStorage::new();
        // アイコン本体をオブジェクトストレージに用意しておく。
        os.put("icon-key.png", vec![1, 2, 3, 4]);
        let e26 = MemoryEvents26Api::new();
        let nr = MemoryNotificationRepo::new();
        let settings = MemorySettingsRepo::new();
        let app = ApprovalRequestApp::new(
            &ar, &mr, &ur, &clock, &discord, &os, &e26, &nr, &settings, BASE_URL,
        );

        app.create(
            &user_ctx(),
            group_id(),
            ApprovalRequestType::EditExhibitionInfo {
                description: None,
                icon_key: Some("icon-key.png".to_string()),
            },
            "理由".to_string(),
        )
        .await
        .expect("create should succeed");

        let messages = discord.sent_messages();
        assert_eq!(messages.len(), 1);
        // 取得したアイコン本体が添付されている。
        assert_eq!(messages[0].attachments.len(), 1);
        assert_eq!(messages[0].attachments[0].file_name, "icon-key.png");
        assert_eq!(messages[0].attachments[0].bytes, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn create_without_icon_has_no_attachment() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let ur = MemoryUserRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let os = MemoryObjectStorage::new();
        let e26 = MemoryEvents26Api::new();
        let nr = MemoryNotificationRepo::new();
        let settings = MemorySettingsRepo::new();
        let app = ApprovalRequestApp::new(
            &ar, &mr, &ur, &clock, &discord, &os, &e26, &nr, &settings, BASE_URL,
        );

        app.create(
            &user_ctx(),
            group_id(),
            ApprovalRequestType::EditExhibitionInfo {
                description: None,
                icon_key: None,
            },
            "理由".to_string(),
        )
        .await
        .expect("create should succeed");

        let messages = discord.sent_messages();
        assert!(messages[0].attachments.is_empty());
    }

    /// 申請者を user_repo と membership_repo に登録し、`ActorContext` を返す。
    /// 承認時の反映先(企画番号)は所属団体の ID から決まるため、所属も要る。
    async fn setup_issuer(
        ur: &MemoryUserRepo,
        mr: &MemoryMembershipRepo,
        now: chrono::DateTime<Utc>,
        group_id: &str,
    ) -> (UserId, GroupId, ActorContext) {
        let issuer_id = UserId::new(Uuid::new_v4());
        let email = EmailAddress::new(format!("{}@example.com", Uuid::new_v4())).unwrap();
        let issuer = User::register(
            issuer_id,
            "申請花子".to_string(),
            email,
            MemoryClock::new(now),
        )
        .unwrap();
        ur.insert(&issuer).await.unwrap();
        let group_id = GroupId::from_str(group_id).unwrap();
        let membership = Membership::new(
            group_id,
            issuer_id,
            Role::FirstResponsible,
            &MemoryClock::new(now),
        );
        mr.insert(membership.clone()).await.unwrap();

        (
            issuer_id,
            group_id,
            ActorContext::User {
                user_id: issuer_id,
                name: "申請花子".to_string(),
                memberships: vec![membership],
                group_type: GroupType::GeneralProject,
            },
        )
    }

    #[tokio::test]
    async fn approve_applies_description_and_icon_to_events26() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let ur = MemoryUserRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let os = MemoryObjectStorage::new();
        // 拡張子の無いキーでも中身(PNG のマジックバイト)から形式を決められること。
        os.put(
            "icon-key",
            vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 1, 2],
        );
        let e26 = MemoryEvents26Api::new();
        let nr = MemoryNotificationRepo::new();
        let settings = MemorySettingsRepo::new();
        let app = ApprovalRequestApp::new(
            &ar, &mr, &ur, &clock, &discord, &os, &e26, &nr, &settings, BASE_URL,
        );

        let (_issuer_id, issuer_group, issuer_ctx) = setup_issuer(&ur, &mr, now, "I-123").await;
        let id = app
            .create(
                &issuer_ctx,
                issuer_group,
                ApprovalRequestType::EditExhibitionInfo {
                    description: Some("新しい紹介文".to_string()),
                    icon_key: Some("icon-key".to_string()),
                },
                "理由".to_string(),
            )
            .await
            .expect("create should succeed");

        app.approve(&admin_ctx(), id, None)
            .await
            .expect("approve should succeed");

        assert_eq!(e26.description("I-123").as_deref(), Some("新しい紹介文"));
        let (content_type, image) = e26.icon("I-123").expect("icon should be applied");
        assert_eq!(content_type, "image/png");
        assert_eq!(image.len(), 10);
    }

    #[tokio::test]
    async fn approve_keeps_request_pending_when_events26_fails() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let ur = MemoryUserRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let os = MemoryObjectStorage::new();
        let e26 = MemoryEvents26Api::new();
        let nr = MemoryNotificationRepo::new();
        let settings = MemorySettingsRepo::new();
        let app = ApprovalRequestApp::new(
            &ar, &mr, &ur, &clock, &discord, &os, &e26, &nr, &settings, BASE_URL,
        );

        let (_issuer_id, issuer_group, issuer_ctx) = setup_issuer(&ur, &mr, now, "I-124").await;
        let id = app
            .create(
                &issuer_ctx,
                issuer_group,
                ApprovalRequestType::EditExhibitionInfo {
                    description: Some("新しい紹介文".to_string()),
                    icon_key: None,
                },
                "理由".to_string(),
            )
            .await
            .expect("create should succeed");

        e26.fail_writes();

        let result = app.approve(&admin_ctx(), id, None).await;
        assert!(result.is_err(), "approve should fail when events26 rejects");

        // 反映できなかった申請は審査中のまま残り、やり直せること。
        let stored = ar.find_by_id(id).await.unwrap().unwrap();
        assert_eq!(stored.status(), &ApprovalRequestStatus::Pending);
        // 承認通知も送られない(作成時の 1 通だけ)。
        assert_eq!(discord.sent_messages().len(), 1);
    }

    #[tokio::test]
    async fn approve_and_reject_issue_approval_request_notifications() {
        let now = Utc.timestamp_opt(0, 0).unwrap();
        let ar = MemoryApprovalRequestRepo::new();
        let mr = MemoryMembershipRepo::new();
        let ur = MemoryUserRepo::new();
        let clock = MemoryClock::new(now);
        let discord = MemoryDiscord::new();
        let os = MemoryObjectStorage::new();
        let e26 = MemoryEvents26Api::new();
        let nr = MemoryNotificationRepo::new();
        let settings = MemorySettingsRepo::new();
        let app = ApprovalRequestApp::new(
            &ar, &mr, &ur, &clock, &discord, &os, &e26, &nr, &settings, BASE_URL,
        );

        let (issuer_id, issuer_group, issuer_ctx) = setup_issuer(&ur, &mr, now, "I-125").await;
        let approved = app
            .create(
                &issuer_ctx,
                issuer_group,
                ApprovalRequestType::EditExhibitionInfo {
                    description: Some("新しい紹介文".to_string()),
                    icon_key: None,
                },
                "理由".to_string(),
            )
            .await
            .expect("create should succeed");
        let rejected = app
            .create(
                &issuer_ctx,
                issuer_group,
                ApprovalRequestType::EditExhibitionInfo {
                    description: Some("却下される紹介文".to_string()),
                    icon_key: None,
                },
                "理由".to_string(),
            )
            .await
            .expect("create should succeed");

        app.approve(&admin_ctx(), approved, None)
            .await
            .expect("approve should succeed");
        app.reject(&admin_ctx(), rejected, None)
            .await
            .expect("reject should succeed");

        let notifications = nr.find_all().await.unwrap();
        assert_eq!(notifications.len(), 2);

        // 承認・却下のどちらも申請 ID を指す通知になっている。
        let ids: Vec<ApprovalRequestId> = notifications
            .iter()
            .map(|n| match n.notification_type() {
                NotificationType::ApprovalRequest {
                    approval_request_id,
                } => *approval_request_id,
                other => panic!("unexpected notification type: {other:?}"),
            })
            .collect();
        assert!(ids.contains(&approved));
        assert!(ids.contains(&rejected));

        // 宛先は申請者本人と所属団体の両方。
        for notification in &notifications {
            assert!(
                notification
                    .targets()
                    .contains(&TargetSpecifier::UserId(issuer_id))
            );
            assert!(notification.targets().contains(&TargetSpecifier::GroupId(
                GroupId::from_str("I-125").unwrap()
            )));
        }
    }
}
