//! `/auth/v2/*` — Group(参加団体)向けの新認証 API。
//!
//! legacy の `/auth/v1`(独自 SHA + ステートレス refresh JWT)を置換する。
//! 中身は [`crate::application::auth::AuthApp`](回転セッション + argon2)。
//! `__Host-` Cookie でリフレッシュトークンを配送する(設計は [`CookieConfig`])。

mod admin;
mod dto;
pub mod extract;
mod handlers;

use crate::application::auth::AuthConfig;
use crate::infra::discord_webhook::WebhookDiscord;
use crate::infra::s3_object_storage::S3ObjectStorage;
use crate::infra::sendgrid_email::SendgridEmail;
use crate::infra::sqlite::SqliteApplication;
use crate::util::oidc::OIDCClient;
use jsonwebtoken::DecodingKey;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

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
    /// 自前アクセストークン(RS256)の検証鍵。認証必須エンドポイントで使う。
    pub access_decoding_key: Arc<DecodingKey>,
    /// アクセストークンの期待 iss。
    pub access_iss: String,
    /// CSRF 対策: cookie 認証エンドポイント(refresh/logout)の Origin 許可リスト。
    pub allowed_origins: Arc<Vec<String>>,
    /// 管理者 Keycloak OIDC クライアント。
    pub oidc_client: Arc<OIDCClient>,
    /// admin OIDC のリダイレクト前セッション(CSRF state → PKCE/nonce)。
    pub auth_sessions: Arc<Mutex<HashMap<String, admin::AdminAuthSession>>>,
    /// OIDC トークン交換用 HTTP クライアント。
    pub http_client: reqwest::Client,
}

/// OpenAPI 上の auth タグ。
pub(crate) const AUTH_TAG: &str = "auth";

/// `/auth/v2` 配下にマウントするルータ(utoipa-axum)。
pub fn router() -> OpenApiRouter<AuthV2State> {
    // utoipa-axum の `routes!` は 1 呼び出し内のハンドラを単一 MethodRouter(=単一パス)へ
    // 畳み込む。異なるパスは別々の `.routes()` 呼び出しに分ける必要がある。
    OpenApiRouter::new()
        .routes(routes!(handlers::login))
        .routes(routes!(handlers::refresh))
        .routes(routes!(handlers::logout))
        .routes(routes!(handlers::logout_all))
        .routes(routes!(handlers::activate))
        .routes(routes!(handlers::password_change))
        .routes(routes!(handlers::password_reset))
        .routes(routes!(handlers::password_reset_confirm))
        .routes(routes!(admin::admin_login))
        .routes(routes!(admin::admin_redirect))
}
