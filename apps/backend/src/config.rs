use confy::ConfyError;
use jsonwebtoken::{DecodingKey, EncodingKey};
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use std::fs;
use tracing_core::LevelFilter;

pub fn init_config() -> Result<Config, ConfyError> {
    confy::load("koudaisai-portal", None)
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub logging: Logging,
    pub web: Web,
    pub db: Db,
    pub s3: S3,
    pub sendgrid: Sendgrid,
    pub discord: Discord,
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
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
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
    pub(crate) fn get_jwt_encoding_key(&self) -> jsonwebtoken::errors::Result<EncodingKey> {
        EncodingKey::from_rsa_pem(fs::read(&self.jwt_secret_key_path).unwrap().as_slice())
    }

    pub(crate) fn get_jwt_decoding_key(&self) -> jsonwebtoken::errors::Result<DecodingKey> {
        DecodingKey::from_rsa_pem(fs::read(&self.jwt_public_key_path).unwrap().as_slice())
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
    pub template: SendgridTemplate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendgridTemplate {
    pub password_reset_email_subject: String,
    pub password_reset_email_body: String,
}

impl Default for SendgridTemplate {
    fn default() -> Self {
        Self {
            password_reset_email_subject: "工大祭ポータル - パスワードリセット".to_string(),
            password_reset_email_body: r#"
{{username}} 様

いつも工大祭ポータルをご利用いただきありがとうございます。

パスワードリセットのご依頼を受け付けました。
以下のリンクより新しいパスワードを設定してください。

パスワードリセットリンク:
https://portal.koudaisai.jp/reset-password?token={{reset_token}}

※ このリンクの有効期限は{{expires_at}}です。期限を過ぎると無効になりますのでご注意ください。

※ 本メールは送信専用です。本メールへのご返信には対応しておりません。

ご不明な点がございましたら、工大祭実行委員会までお問い合わせください。

もしこのメールに心当たりがない場合は、このメールを破棄してください。

今後とも工大祭実行委員会をよろしくお願いいたします。

--------------------------------
東京科学大学　工大祭実行委員会
工大祭ポータル
https://portal.koudaisai.jp/

お問い合わせ先
　模擬店企画/一般企画 sanka@koudaisai.jp
　ステージ企画 stage@koudaisai.jp
　研究室公開企画 laboratory@koudaisai.jp
            "#
            .trim()
            .parse()
            .unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Discord {
    pub approval_request_url: String,
}
