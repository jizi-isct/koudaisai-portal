pub mod approval_request_repo_impl;
pub mod clock_impl;
pub mod email_impl;
pub mod group_repo_impl;
pub mod membership_repo_impl;
pub mod notification_repo_impl;
pub mod transaction_impl;
pub mod user_repo_impl;

use crate::application::Application;
use crate::infra::memory::approval_request_repo_impl::MemoryApprovalRequestRepo;
use crate::infra::memory::clock_impl::MemoryClock;
use crate::infra::memory::email_impl::MemoryEmail;
use crate::infra::memory::group_repo_impl::MemoryGroupRepo;
use crate::infra::memory::membership_repo_impl::MemoryMembershipRepo;
use crate::infra::memory::transaction_impl::MemoryTransaction;
use crate::infra::memory::user_repo_impl::MemoryUserRepo;

pub type MemoryApplication = Application<
    MemoryTransaction,
    MemoryApprovalRequestRepo,
    MemoryGroupRepo,
    MemoryMembershipRepo,
    MemoryUserRepo,
    MemoryClock,
    MemoryEmail,
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
            MemoryClock::new(now),
            MemoryEmail::new(),
        )
    }
}
