use crate::application::error::FindError;
use crate::application::ports::access_token_issuer::AccessTokenIssuer;
use crate::application::ports::clock::Clock;
use crate::application::ports::discord::Discord;
use crate::application::ports::email::Email;
use crate::application::ports::events26_api::Events26Api;
use crate::application::ports::meta_fetcher::MetaFetcher;
use crate::application::ports::object_storage::ObjectStorage;
use crate::application::ports::password_hasher::PasswordHasher;
use crate::application::ports::repositories::approval_request_repo::ApprovalRequestRepo;
use crate::application::ports::repositories::document_category_repo::DocumentCategoryRepo;
use crate::application::ports::repositories::document_repo::DocumentRepo;
use crate::application::ports::repositories::form_repo::FormRepo;
use crate::application::ports::repositories::group_repo::GroupRepo;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::ports::repositories::notification_repo::NotificationRepo;
use crate::application::ports::repositories::one_time_token_repo::OneTimeTokenRepo;
use crate::application::ports::repositories::session_repo::SessionRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::ports::secret_generator::SecretGenerator;
use crate::application::transaction::Transaction;
use crate::application::user::UserApp;
use crate::domain::actor_ctx::ActorContext;
use crate::domain::user::User;
use crate::domain::user_id::UserId;

pub mod approval_request;
pub mod auth;
pub mod authz;
pub mod document;
pub mod document_category;
pub mod error;
pub mod events26;
pub mod file;
pub mod form;
pub mod group;
pub mod meta;
pub mod notification;
pub mod ports;
pub mod transaction;
pub mod user;

pub struct Application<
    Tx: Transaction,
    AR: ApprovalRequestRepo<Tx>,
    GR: GroupRepo<Tx>,
    MR: MembershipRepo<Tx>,
    UR: UserRepo<Tx>,
    DR: DocumentRepo<Tx>,
    DCR: DocumentCategoryRepo<Tx>,
    FR: FormRepo,
    C: Clock,
    E: Email,
    OS: ObjectStorage,
    D: Discord,
    SR: SessionRepo<Tx>,
    OTR: OneTimeTokenRepo<Tx>,
    PH: PasswordHasher,
    SG: SecretGenerator,
    ATI: AccessTokenIssuer,
    NR: NotificationRepo<Tx>,
    MF: MetaFetcher,
    EA: Events26Api,
> {
    _phantom: std::marker::PhantomData<Tx>,
    approval_request_repo: AR,
    group_repo: GR,
    membership_repo: MR,
    user_repo: UR,
    document_repo: DR,
    document_category_repo: DCR,
    form_repo: FR,
    clock: C,
    // Email ポートは配線済みだが現状 app 層に消費者がいない。将来の通知メール送出で利用予定。
    #[allow(dead_code)]
    email: E,
    object_storage: OS,
    discord: D,
    session_repo: SR,
    one_time_token_repo: OTR,
    password_hasher: PH,
    secret_generator: SG,
    access_token_issuer: ATI,
    notification_repo: NR,
    meta_fetcher: MF,
    /// 企画情報API(events26)クライアント。企画情報の編集申請を承認したときに
    /// 企画へ反映するため、承認のユースケースから使う。
    events26_api: EA,
    /// 通知の本文などで使う公開ベース URL(例: `https://portal.koudaisai.jp`)。
    base_url: String,
}

impl<
    Tx: Transaction + Send,
    AR: ApprovalRequestRepo<Tx>,
    GR: GroupRepo<Tx>,
    MR: MembershipRepo<Tx>,
    UR: UserRepo<Tx> + Send + Sync,
    DR: DocumentRepo<Tx>,
    DCR: DocumentCategoryRepo<Tx>,
    FR: FormRepo,
    C: Clock + Send + Sync,
    E: Email,
    OS: ObjectStorage,
    D: Discord,
    SR: SessionRepo<Tx> + Send + Sync,
    OTR: OneTimeTokenRepo<Tx> + Send + Sync,
    PH: PasswordHasher,
    SG: SecretGenerator,
    ATI: AccessTokenIssuer,
    NR: NotificationRepo<Tx>,
    MF: MetaFetcher,
    EA: Events26Api,
> Application<Tx, AR, GR, MR, UR, DR, DCR, FR, C, E, OS, D, SR, OTR, PH, SG, ATI, NR, MF, EA>
{
    // 全リポジトリ/ポートを束ねる合成ルート。引数が多いのは設計上不可避。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        approval_request_repo: AR,
        group_repo: GR,
        membership_repo: MR,
        user_repo: UR,
        document_repo: DR,
        document_category_repo: DCR,
        form_repo: FR,
        clock: C,
        email: E,
        object_storage: OS,
        discord: D,
        session_repo: SR,
        one_time_token_repo: OTR,
        password_hasher: PH,
        secret_generator: SG,
        access_token_issuer: ATI,
        notification_repo: NR,
        meta_fetcher: MF,
        events26_api: EA,
        base_url: String,
    ) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            approval_request_repo,
            group_repo,
            membership_repo,
            user_repo,
            document_repo,
            document_category_repo,
            form_repo,
            clock,
            email,
            object_storage,
            discord,
            session_repo,
            one_time_token_repo,
            password_hasher,
            secret_generator,
            access_token_issuer,
            notification_repo,
            meta_fetcher,
            events26_api,
            base_url,
        }
    }

    pub fn approval_request(
        &'_ self,
    ) -> approval_request::ApprovalRequestApp<'_, Tx, AR, MR, UR, C, D, OS, EA> {
        approval_request::ApprovalRequestApp::new(
            &self.approval_request_repo,
            &self.membership_repo,
            &self.user_repo,
            &self.clock,
            &self.discord,
            &self.object_storage,
            &self.events26_api,
            &self.base_url,
        )
    }

    pub fn events26(&'_ self) -> events26::Events26App<'_, EA> {
        events26::Events26App::new(&self.events26_api)
    }

    pub fn group(&'_ self) -> group::GroupApp<'_, Tx, GR, MR, UR, C> {
        group::GroupApp::new(
            &self.group_repo,
            &self.membership_repo,
            &self.user_repo,
            &self.clock,
        )
    }

    pub fn user(&'_ self) -> UserApp<'_, Tx, MR, UR, C> {
        UserApp::new(&self.membership_repo, &self.user_repo, &self.clock)
    }

    pub fn document(&'_ self) -> document::DocumentApp<'_, Tx, DR, C> {
        document::DocumentApp::new(&self.document_repo, &self.clock)
    }

    pub fn document_category(&'_ self) -> document_category::DocumentCategoryApp<'_, Tx, DCR, C> {
        document_category::DocumentCategoryApp::new(&self.document_category_repo, &self.clock)
    }

    pub fn form(&'_ self) -> form::FormApp<'_, FR, C> {
        form::FormApp::new(&self.form_repo, &self.clock)
    }

    pub fn file(&'_ self) -> file::FileApp<'_, OS> {
        file::FileApp::new(&self.object_storage)
    }

    pub fn notification(&'_ self) -> notification::NotificationApp<'_, Tx, NR, C> {
        notification::NotificationApp::new(&self.notification_repo, &self.clock)
    }

    pub fn meta(&'_ self) -> meta::MetaApp<'_, MF> {
        meta::MetaApp::new(&self.meta_fetcher)
    }

    /// 認証ユースケース。`config` と定数時間ログイン用ダミー PHC は
    /// 呼び出し側(State)が現行設定から渡す。
    pub fn auth(&'_ self, config: auth::AuthConfig, dummy_phc: String) -> auth::AuthApp<'_, Tx, C> {
        auth::AuthApp::new(
            &self.user_repo,
            &self.one_time_token_repo,
            &self.session_repo,
            &self.password_hasher,
            &self.secret_generator,
            &self.access_token_issuer,
            &self.clock,
            config,
            dummy_phc,
        )
    }

    /// 認証(authn)のためにユーザを直接ロードする。authz は行わない
    /// (アクセストークンのゲート判定や ActorContext 構築に用いる)。
    pub async fn find_user_for_auth(&self, user_id: UserId) -> Result<Option<User>, FindError> {
        self.user_repo.find_by_id(user_id).await
    }

    /// 認証済みユーザの `ActorContext::User` を組み立てる(authn→authz のブリッジ)。
    /// グループ種別は最初の所属グループから解決する(approval_request 等と同じ慣習)。
    /// ユーザ不在 / 所属グループ無し / グループ不整合の場合は `None`。
    pub async fn build_actor_context(
        &self,
        user_id: UserId,
    ) -> Result<Option<ActorContext>, FindError> {
        let Some(user) = self.user_repo.find_by_id(user_id).await? else {
            return Ok(None);
        };
        let memberships = self.membership_repo.find_by_user_id(user_id).await?;
        let Some(first) = memberships.first() else {
            return Ok(None);
        };
        let Some(group) = self.group_repo.find_by_id(first.group_id()).await? else {
            return Ok(None);
        };
        let group_type = *group.r#type();
        Ok(Some(ActorContext::User {
            user_id,
            name: user.name().to_string(),
            memberships,
            group_type,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::email_address::EmailAddress;
    use crate::domain::group::GroupType;
    use crate::domain::group_id::GroupId;
    use crate::domain::membership::Role;
    use crate::infra::memory::MemoryApplication;
    use crate::infra::memory::transaction_impl::MemoryTransaction;
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;

    fn admin() -> ActorContext {
        ActorContext::Admin {
            user_id: UserId::new(Uuid::new_v4()),
            name: "admin".to_string(),
            claims: vec![
                "koudaisai-portal:admin:user:read".to_string(),
                "koudaisai-portal:admin:group:create".to_string(),
                "koudaisai-portal:admin:group:update".to_string(),
            ],
        }
    }

    #[tokio::test]
    async fn build_actor_context_resolves_user_and_group_type() {
        let app =
            MemoryApplication::new_memory_app(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let admin = admin();
        let user_id = UserId::new(Uuid::new_v4());
        app.user()
            .register(
                &admin,
                user_id,
                "User".to_string(),
                EmailAddress::new("g@example.com".to_string()).unwrap(),
            )
            .await
            .unwrap();

        let group_id = GroupId::new('P', 7).unwrap();
        app.group()
            .create_group(
                &admin,
                MemoryTransaction::new(),
                group_id,
                "Press".to_string(),
                GroupType::Press,
            )
            .await
            .unwrap();
        app.group()
            .add_member(
                &admin,
                MemoryTransaction::new(),
                group_id,
                user_id,
                Role::Representative,
            )
            .await
            .unwrap();

        let actor = app
            .build_actor_context(user_id)
            .await
            .unwrap()
            .expect("user with a group resolves to a User actor");
        match actor {
            ActorContext::User {
                user_id: uid,
                group_type,
                memberships,
                ..
            } => {
                assert_eq!(uid, user_id);
                assert_eq!(group_type, GroupType::Press);
                assert_eq!(memberships.len(), 1);
            }
            _ => panic!("expected ActorContext::User"),
        }
    }

    #[tokio::test]
    async fn build_actor_context_none_for_groupless_or_missing_user() {
        let app =
            MemoryApplication::new_memory_app(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let admin = admin();
        let user_id = UserId::new(Uuid::new_v4());
        app.user()
            .register(
                &admin,
                user_id,
                "U".to_string(),
                EmailAddress::new("x@example.com".to_string()).unwrap(),
            )
            .await
            .unwrap();
        // 所属グループ無し。
        assert!(app.build_actor_context(user_id).await.unwrap().is_none());
        // 不在ユーザ。
        assert!(
            app.build_actor_context(UserId::new(Uuid::new_v4()))
                .await
                .unwrap()
                .is_none()
        );
    }
}
