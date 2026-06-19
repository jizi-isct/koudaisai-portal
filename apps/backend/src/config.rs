use confy::ConfyError;
use jsonwebtoken::{DecodingKey, EncodingKey};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{env, fs, path::PathBuf};
use tracing_core::LevelFilter;

pub const CONFIG_PATH_ENV: &str = "KOUDAISAI_PORTAL_CONFIG_PATH";

pub fn init_config() -> Result<Config, ConfyError> {
    match env::var_os(CONFIG_PATH_ENV).filter(|path| !path.is_empty()) {
        Some(path) => confy::load_path(PathBuf::from(path)),
        None => confy::load("koudaisai-portal", None),
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub logging: Logging,
    pub web: Web,
    pub db: Db,
    pub s3: S3,
    pub sendgrid: Sendgrid,
    pub discord: Discord,
    pub secrets: Secrets,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Logging {
    pub log_level: LogLevel,
    pub json: bool,
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            log_level: LogLevel::default(),
            json: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Default)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}


impl LogLevel {
    pub fn to_level_filter(&self) -> LevelFilter {
        match self {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Web {
    pub server: Server,
    pub auth: Auth,
    pub static_files: StaticFiles,
    /// v3 認証(argon2 + 回転セッション)の設定。既存 config を壊さないため `serde(default)`。
    #[serde(default)]
    pub auth_v3: AuthV3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub base_url: String,
    pub host: String,
    pub port: u16,
}
impl Default for Server {
    fn default() -> Self {
        Self {
            base_url: "https://portal.koudaisai.jp".to_string(),
            host: "0.0.0.0".parse().unwrap(),
            port: 8080,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub password_salt: String,
    pub activation_salt: String,
    pub stretch_cost: u8,
    pub jwt_secret_key_path: String,
    pub jwt_public_key_path: String,
    pub keycloak: KeyCloak,
}

impl Default for Auth {
    fn default() -> Self {
        let mut rng = rand::rng();
        Self {
            password_salt: Alphanumeric.sample_string(&mut rng, 16),
            activation_salt: Alphanumeric.sample_string(&mut rng, 16),
            stretch_cost: 13,
            jwt_secret_key_path: "./secret_key".parse().unwrap(),
            jwt_public_key_path: "./public_key".parse().unwrap(),
            keycloak: KeyCloak::default(),
        }
    }
}

impl Auth {
    pub fn get_jwt_encoding_key(&self) -> jsonwebtoken::errors::Result<EncodingKey> {
        EncodingKey::from_rsa_pem(fs::read(&self.jwt_secret_key_path).unwrap().as_slice())
    }

    pub fn get_jwt_decoding_key(&self) -> jsonwebtoken::errors::Result<DecodingKey> {
        DecodingKey::from_rsa_pem(fs::read(&self.jwt_public_key_path).unwrap().as_slice())
    }
}

/// v3 認証の設定。値が無い場合は OWASP/設計既定値にフォールバックする。
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct AuthV3 {
    pub argon2_m_cost_kib: u32,
    pub argon2_t_cost: u32,
    pub argon2_p_cost: u32,
    pub argon2_output_len: usize,
    /// argon2 同時実行数の上限(メモリ DoS 対策)。
    pub argon2_max_parallelism: usize,
    pub password_min_length: usize,
    pub password_max_length: usize,
    pub access_token_ttl_secs: i64,
    pub session_absolute_ttl_secs: i64,
    pub session_idle_ttl_secs: i64,
    pub max_sessions_per_user: usize,
    pub activation_token_ttl_secs: i64,
    pub reset_token_ttl_secs: i64,
    pub refresh_cookie_name: String,
    /// `__Host-` は Secure を要求するため本番(https)は true。
    pub refresh_cookie_secure: bool,
    pub refresh_cookie_same_site: String,
    /// パスワードリセットリンクのベース URL。
    pub reset_link_base: String,
    /// 自前アクセストークンの iss。
    pub access_token_iss: String,
    /// CORS 許可オリジン(credentials 付きのため `*` 不可)。
    pub cors_allowed_origins: Vec<String>,
}

impl Default for AuthV3 {
    fn default() -> Self {
        Self {
            argon2_m_cost_kib: 19456,
            argon2_t_cost: 2,
            argon2_p_cost: 1,
            argon2_output_len: 32,
            argon2_max_parallelism: 4,
            password_min_length: 12,
            password_max_length: 1024,
            access_token_ttl_secs: 300,
            session_absolute_ttl_secs: 60 * 60 * 24 * 183, // ~6 か月
            session_idle_ttl_secs: 60 * 60 * 24 * 183,
            max_sessions_per_user: 10,
            activation_token_ttl_secs: 60 * 60 * 48,
            reset_token_ttl_secs: 60 * 30,
            refresh_cookie_name: "__Host-refresh_token".to_string(),
            refresh_cookie_secure: true,
            refresh_cookie_same_site: "Strict".to_string(),
            reset_link_base: "https://portal.koudaisai.jp/password/reset".to_string(),
            access_token_iss: "https://portal.koudaisai.jp".to_string(),
            cors_allowed_origins: vec!["https://portal.koudaisai.jp".to_string()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KeyCloak {
    pub id: String,
    pub secret: String,
    pub issuer: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Db {
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaticFiles {
    pub web_path: String,
    pub admin_path: String,
}

impl Default for StaticFiles {
    fn default() -> Self {
        Self {
            web_path: "/var/www/html/web".into(),
            admin_path: "/var/www/html/admin".into(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct S3 {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub endpoint: String,
    pub bucket: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Sendgrid {
    pub sender_address: String,
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Discord {
    pub approval_request_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Secrets {
    pub plans_info_api_client_id: Secret,
    pub plans_info_api_client_secret: Secret,
    /// 回転セッションの secret をハッシュする HMAC ペッパー。DB には出さない。
    #[serde(default)]
    pub session_secret_pepper: Secret,
}

/// 機密情報のラッパー。`Debug` でマスクし、誤ってログに秘密が漏れるのを防ぐ。
#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Secret {
    String(String),
}

impl Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Secret::String(_) => write!(f, "***"),
        }
    }
}

impl Default for Secret {
    fn default() -> Self {
        Secret::String("".to_string())
    }
}

impl From<Secret> for String {
    fn from(secret: Secret) -> Self {
        match secret {
            Secret::String(s) => s,
        }
    }
}

impl AsRef<str> for Secret {
    fn as_ref(&self) -> &str {
        match self {
            Secret::String(s) => s.as_str(),
        }
    }
}
