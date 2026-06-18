pub mod access_token_issuer_impl;
pub mod approval_request_repo_impl;
pub mod clock_impl;
pub mod discord_impl;
pub mod document_category_repo_impl;
pub mod document_repo_impl;
pub mod email_impl;
pub mod form_repo_impl;
pub mod group_repo_impl;
pub mod membership_repo_impl;
pub mod notification_repo_impl;
pub mod object_storage_impl;
pub mod one_time_token_repo_impl;
pub mod password_hasher_impl;
pub mod secret_generator_impl;
pub mod session_repo_impl;
pub mod transaction_impl;
pub mod user_repo_impl;

use crate::application::Application;
use crate::infra::memory::access_token_issuer_impl::MemoryAccessTokenIssuer;
use crate::infra::memory::approval_request_repo_impl::MemoryApprovalRequestRepo;
use crate::infra::memory::clock_impl::MemoryClock;
use crate::infra::memory::discord_impl::MemoryDiscord;
use crate::infra::memory::document_category_repo_impl::MemoryDocumentCategoryRepo;
use crate::infra::memory::document_repo_impl::MemoryDocumentRepo;
use crate::infra::memory::email_impl::MemoryEmail;
use crate::infra::memory::form_repo_impl::MemoryFormRepo;
use crate::infra::memory::group_repo_impl::MemoryGroupRepo;
use crate::infra::memory::membership_repo_impl::MemoryMembershipRepo;
use crate::infra::memory::object_storage_impl::MemoryObjectStorage;
use crate::infra::memory::one_time_token_repo_impl::MemoryOneTimeTokenRepo;
use crate::infra::memory::password_hasher_impl::MemoryPasswordHasher;
use crate::infra::memory::secret_generator_impl::MemorySecretGenerator;
use crate::infra::memory::session_repo_impl::MemorySessionRepo;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use crate::infra::memory::user_repo_impl::MemoryUserRepo;

pub type MemoryApplication = Application<
    MemoryTransaction,
    MemoryApprovalRequestRepo,
    MemoryGroupRepo,
    MemoryMembershipRepo,
    MemoryUserRepo,
    MemoryDocumentRepo,
    MemoryDocumentCategoryRepo,
    MemoryFormRepo,
    MemoryClock,
    MemoryEmail,
    MemoryObjectStorage,
    MemoryDiscord,
    MemorySessionRepo,
    MemoryOneTimeTokenRepo,
    MemoryPasswordHasher,
    MemorySecretGenerator,
    MemoryAccessTokenIssuer,
>;

impl MemoryApplication {
    /// テスト用の `MemoryApplication` インスタンスを生成します。
    /// すべてのリポジトリやメール送信機能はインメモリで動作します。
    /// `now` には初期時刻を指定してください。
    pub fn new_memory_app(now: chrono::DateTime<chrono::Utc>) -> Self {
        Application::new(
            MemoryApprovalRequestRepo::new(),
            MemoryGroupRepo::new(),
            MemoryMembershipRepo::new(),
            MemoryUserRepo::new(),
            MemoryDocumentRepo::new(),
            MemoryDocumentCategoryRepo::new(),
            MemoryFormRepo::new(),
            MemoryClock::new(now),
            MemoryEmail::new(),
            MemoryObjectStorage::new(),
            MemoryDiscord::new(),
            MemorySessionRepo::new(),
            MemoryOneTimeTokenRepo::new(),
            MemoryPasswordHasher::new(),
            MemorySecretGenerator::new(),
            MemoryAccessTokenIssuer::new(),
            "http://localhost".to_string(),
        )
    }
}
