//! SQLite(sqlx)バックエンドのインフラ実装。
//!
//! 各リポジトリは `SqlitePool` を保持し，トランザクション外の操作はプールへ，
//! `*_in` 操作は [`SqliteTransaction`] が保持する接続へクエリを発行する。
//! ドメイン知識は domain/application 層に閉じており，ここではドメインモデルと
//! テーブルの行表現を相互変換するだけにとどめる。

pub mod approval_request_repo_impl;
pub mod document_category_repo_impl;
pub mod document_repo_impl;
pub mod form_repo_impl;
pub mod group_repo_impl;
pub mod membership_repo_impl;
pub mod notification_repo_impl;
pub mod one_time_token_repo_impl;
pub mod session_repo_impl;
pub mod transaction_impl;
pub mod user_repo_impl;
mod util;

#[cfg(test)]
mod tests;

use crate::application::Application;
use crate::application::ports::discord::Discord;
use crate::application::ports::email::Email;
use crate::application::ports::object_storage::ObjectStorage;
use crate::infra::argon2_password_hasher::Argon2PasswordHasher;
use crate::infra::clock_impl::ClockImpl;
use crate::infra::jwt_access_token_issuer::JwtAccessTokenIssuer;
use crate::infra::random_secret_generator::RandomSecretGenerator;
use crate::infra::sqlite::approval_request_repo_impl::SqliteApprovalRequestRepo;
use crate::infra::sqlite::document_category_repo_impl::SqliteDocumentCategoryRepo;
use crate::infra::sqlite::document_repo_impl::SqliteDocumentRepo;
use crate::infra::sqlite::form_repo_impl::SqliteFormRepo;
use crate::infra::sqlite::group_repo_impl::SqliteGroupRepo;
use crate::infra::sqlite::membership_repo_impl::SqliteMembershipRepo;
use crate::infra::sqlite::notification_repo_impl::SqliteNotificationRepo;
use crate::infra::sqlite::one_time_token_repo_impl::SqliteOneTimeTokenRepo;
use crate::infra::reqwest_meta_fetcher::ReqwestMetaFetcher;
use crate::infra::sqlite::session_repo_impl::SqliteSessionRepo;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::user_repo_impl::SqliteUserRepo;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

/// SQLite バックエンドで構成した [`Application`]。
/// メール送信ポート `E`・Discord 通知ポート `D` は外部サービス実装
/// (SendGrid / webhook 等)を注入する。
pub type SqliteApplication<E, OS, D> = Application<
    SqliteTransaction,
    SqliteApprovalRequestRepo,
    SqliteGroupRepo,
    SqliteMembershipRepo,
    SqliteUserRepo,
    SqliteDocumentRepo,
    SqliteDocumentCategoryRepo,
    SqliteFormRepo,
    ClockImpl,
    E,
    OS,
    D,
    SqliteSessionRepo,
    SqliteOneTimeTokenRepo,
    Argon2PasswordHasher,
    RandomSecretGenerator,
    JwtAccessTokenIssuer,
    SqliteNotificationRepo,
    ReqwestMetaFetcher,
>;

/// SQLite プールを生成し，マイグレーションを適用する。
///
/// 接続ごとに `PRAGMA foreign_keys = ON` を効かせ，DB ファイルが無ければ作成する。
pub async fn connect_and_migrate(database_url: &str) -> anyhow::Result<SqlitePool> {
    let opts = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new().connect_with(opts).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

/// プールと外部サービス実装(メール・Discord)、公開ベース URL から
/// [`SqliteApplication`] を組み立てる。
#[allow(clippy::too_many_arguments)]
pub fn new_sqlite_application<E: Email, OS: ObjectStorage, D: Discord>(
    pool: SqlitePool,
    email: E,
    object_storage: OS,
    discord: D,
    base_url: String,
    password_hasher: Argon2PasswordHasher,
    secret_generator: RandomSecretGenerator,
    access_token_issuer: JwtAccessTokenIssuer,
) -> SqliteApplication<E, OS, D> {
    Application::new(
        SqliteApprovalRequestRepo::new(pool.clone()),
        SqliteGroupRepo::new(pool.clone()),
        SqliteMembershipRepo::new(pool.clone()),
        SqliteUserRepo::new(pool.clone()),
        SqliteDocumentRepo::new(pool.clone()),
        SqliteDocumentCategoryRepo::new(pool.clone()),
        SqliteFormRepo::new(pool.clone()),
        ClockImpl,
        email,
        object_storage,
        discord,
        SqliteSessionRepo::new(pool.clone()),
        SqliteOneTimeTokenRepo::new(pool.clone()),
        password_hasher,
        secret_generator,
        access_token_issuer,
        SqliteNotificationRepo::new(pool.clone()),
        ReqwestMetaFetcher::from_config(),
        base_url,
    )
}
