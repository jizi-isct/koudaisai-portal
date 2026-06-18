use crate::application::ports::access_token_issuer::AccessTokenIssuer;
use crate::application::ports::clock::Clock;
use crate::application::ports::discord::Discord;
use crate::application::ports::password_hasher::PasswordHasher;
use crate::application::ports::secret_generator::SecretGenerator;
use crate::application::ports::email::Email;
use crate::application::ports::object_storage::ObjectStorage;
use crate::application::ports::repositories::approval_request_repo::ApprovalRequestRepo;
use crate::application::ports::repositories::document_category_repo::DocumentCategoryRepo;
use crate::application::ports::repositories::document_repo::DocumentRepo;
use crate::application::ports::repositories::form_repo::FormRepo;
use crate::application::ports::repositories::group_repo::GroupRepo;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::ports::repositories::one_time_token_repo::OneTimeTokenRepo;
use crate::application::ports::repositories::session_repo::SessionRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::error::FindError;
use crate::application::transaction::Transaction;
use crate::application::user::UserApp;
use crate::domain::user::User;
use crate::domain::user_id::UserId;

pub mod approval_request;
pub mod auth;
pub mod authz;
pub mod document;
pub mod document_category;
pub mod error;
pub mod file;
pub mod form;
pub mod group;
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
    email: E,
    object_storage: OS,
    discord: D,
    session_repo: SR,
    one_time_token_repo: OTR,
    password_hasher: PH,
    secret_generator: SG,
    access_token_issuer: ATI,
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
> Application<Tx, AR, GR, MR, UR, DR, DCR, FR, C, E, OS, D, SR, OTR, PH, SG, ATI>
{
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
            base_url,
        }
    }

    pub fn approval_request(
        &'_ self,
    ) -> approval_request::ApprovalRequestApp<'_, Tx, AR, MR, UR, C, D, OS> {
        approval_request::ApprovalRequestApp::new(
            &self.approval_request_repo,
            &self.membership_repo,
            &self.user_repo,
            &self.clock,
            &self.discord,
            &self.object_storage,
            &self.base_url,
        )
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

    /// 認証ユースケース。`config` と定数時間ログイン用ダミー PHC は
    /// 呼び出し側(State)が現行設定から渡す。
    pub fn auth(
        &'_ self,
        config: auth::AuthConfig,
        dummy_phc: String,
    ) -> auth::AuthApp<'_, Tx, C> {
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
}
