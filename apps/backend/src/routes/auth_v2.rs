//! `/auth/v2/*` — Group(参加団体)向けの新認証 API。
//!
//! legacy の `/auth/v1`(独自 SHA + ステートレス refresh JWT)を置換する。
//! 中身は [`crate::application::auth::AuthApp`](回転セッション + argon2)。
//! `__Host-` Cookie でリフレッシュトークンを配送する(設計は [`CookieConfig`])。

mod dto;
mod handlers;

use crate::application::auth::AuthConfig;
use crate::infra::discord_webhook::WebhookDiscord;
use crate::infra::s3_object_storage::S3ObjectStorage;
use crate::infra::sendgrid_email::SendgridEmail;
use crate::infra::sqlite::SqliteApplication;
use axum::Router;
use axum::routing::post;
use sqlx::SqlitePool;
use std::sync::Arc;

/// 本番構成の具体 [`Application`](crate::application::Application)。
/// auth 関連ポートは [`SqliteApplication`] が argon2 / HMAC / JWT 実装で固定する。
pub type ProdApplication = SqliteApplication<SendgridEmail, S3ObjectStorage, WebhookDiscord>;

/// リフレッシュトークン Cookie の属性。`__Host-` 前提(Secure・Domain なし・Path=/)。
#[derive(Clone)]
pub struct CookieConfig {
    /// 例: `__Host-refresh_token`。
    pub name: String,
    /// 本番(https)は true。`__Host-` は Secure を要求するため必須。
    pub secure: bool,
    /// `Strict` 既定。
    pub same_site: String,
}

/// `/auth/v2` ハンドラが共有する状態。
#[derive(Clone)]
pub struct AuthV2State {
    pub app: Arc<ProdApplication>,
    /// トランザクション生成用(`SqliteTransaction::new(pool)`)。
    pub pool: SqlitePool,
    pub auth_config: AuthConfig,
    /// 定数時間ログイン用ダミー PHC。**composition root で本番 argon2 から生成すること**。
    pub dummy_phc: Arc<String>,
    pub cookie: CookieConfig,
    /// パスワードリセットメール送信用(リクエストパス外で spawn して使う)。
    pub email: Arc<SendgridEmail>,
    /// リセットリンクのベース URL(例: `https://portal.koudaisai.jp/password/reset`)。
    pub reset_link_base: String,
}

/// `/auth/v2` 配下にマウントするルータ。
pub fn router() -> Router<AuthV2State> {
    Router::new()
        .route("/login", post(handlers::login))
        .route("/refresh", post(handlers::refresh))
        .route("/logout", post(handlers::logout))
        .route("/activate", post(handlers::activate))
        .route("/password/reset", post(handlers::password_reset))
        .route(
            "/password/reset/confirm",
            post(handlers::password_reset_confirm),
        )
}
