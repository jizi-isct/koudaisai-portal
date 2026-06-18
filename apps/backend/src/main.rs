// TODO(sqlx移行): Db 設定型は init_db 復活時に再度 import する
use crate::application::auth::AuthConfig;
use crate::application::ports::password_hasher::PasswordHasher;
use crate::application::ports::secret_generator::SecretGenerator;
use crate::config::{Logging, init_config};
use crate::domain::plaintext_password::PlaintextPassword;
use crate::infra::argon2_password_hasher::Argon2PasswordHasher;
use crate::infra::discord_webhook::WebhookDiscord;
use crate::infra::jwt_access_token_issuer::JwtAccessTokenIssuer;
use crate::infra::random_secret_generator::RandomSecretGenerator;
use crate::infra::s3_object_storage::S3ObjectStorage;
use crate::infra::sendgrid_email::SendgridEmail;
use crate::infra::sqlite::{connect_and_migrate, new_sqlite_application};
use crate::routes::auth_v2::{AuthV2State, CookieConfig};
use crate::routes_legacy::init_routes;
use crate::service::discord::Discord;
use crate::util::oidc::OIDCClient;
use chrono::Duration;
use jsonwebtoken::Algorithm;
use std::net::SocketAddr;
use std::sync::Arc;
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::Credentials;
// TODO(sqlx移行): sea-orm の Migrator/Database はsqlx移行で application 層へ再配線する
// use migration::{Migrator, MigratorTrait};
use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
// use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::{info, instrument};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub(crate) mod application;
pub mod config;
pub(crate) mod domain;
// TODO(sqlx移行): entities/sea_orm_entities 層は撤去。DBアクセスは application 層へ再配線
// pub mod entities;
mod infra;
pub mod middlewares;
pub mod routes;
mod routes_legacy;
mod service;
pub mod util;

const MAJOR_VERSION: u32 = pkg_version_major!();
const MINOR_VERSION: u32 = pkg_version_minor!();
const PATCH_VERSION: u32 = pkg_version_patch!();
#[tokio::main]
async fn main() {
    //初期化
    let config = init_config().unwrap();
    init_logging(config.logging);
    info!(
        "Koudaisai Portal v{}.{}.{} (c) 2025 JIZI All Rights Reserved.",
        MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION
    );

    // openid connect init
    let oidc_client = init_oidc(
        config.web.auth.keycloak.id.clone(),
        config.web.auth.keycloak.secret.clone(),
        config.web.auth.keycloak.issuer.to_string(),
        format!("{}{}", &config.web.server.base_url, "/login"),
    )
    .await;

    // s3 init
    let s3_client = init_s3(
        config.s3.access_key_id.clone(),
        config.s3.secret_access_key.clone(),
        config.s3.endpoint.clone(),
    )
    .await;

    // ===== v3 認証(/auth/v2)の composition root =====
    let pool = connect_and_migrate(&config.db.address).await.unwrap();
    let v3 = config.web.auth_v3.clone();

    let password_hasher = Argon2PasswordHasher::new(
        v3.argon2_m_cost_kib,
        v3.argon2_t_cost,
        v3.argon2_p_cost,
        v3.argon2_output_len,
        v3.argon2_max_parallelism,
    );
    let secret_generator = RandomSecretGenerator::new(
        config
            .secrets
            .session_secret_pepper
            .as_ref()
            .as_bytes()
            .to_vec(),
    );
    let access_issuer = JwtAccessTokenIssuer::new(
        Algorithm::RS256,
        v3.access_token_iss.clone(),
        config.web.auth.get_jwt_encoding_key().unwrap(),
    );

    // 定数時間ログイン用ダミー PHC は本番 argon2 で乱数をハッシュして生成する(監査 Low 対応)。
    let dummy_phc = {
        let plain = PlaintextPassword::new(secret_generator.generate_secret())
            .expect("generated secret satisfies length policy");
        password_hasher.hash(&plain).await.expect("hash dummy phc")
    };

    let prod_app = new_sqlite_application(
        pool.clone(),
        SendgridEmail::new(
            config.sendgrid.api_key.clone(),
            config.sendgrid.sender_address.clone(),
        ),
        S3ObjectStorage::new(s3_client.clone(), config.s3.bucket.clone()),
        WebhookDiscord::new(config.discord.approval_request_url.clone()),
        config.web.server.base_url.clone(),
        password_hasher,
        secret_generator,
        access_issuer,
    );

    let auth_v2_state = AuthV2State {
        app: Arc::new(prod_app),
        pool: pool.clone(),
        auth_config: AuthConfig {
            access_ttl: Duration::seconds(v3.access_token_ttl_secs),
            session_absolute_ttl: Duration::seconds(v3.session_absolute_ttl_secs),
            session_idle_ttl: Duration::seconds(v3.session_idle_ttl_secs),
            max_sessions_per_user: v3.max_sessions_per_user,
            activation_ttl: Duration::seconds(v3.activation_token_ttl_secs),
            reset_ttl: Duration::seconds(v3.reset_token_ttl_secs),
        },
        dummy_phc: Arc::new(dummy_phc),
        cookie: CookieConfig {
            name: v3.refresh_cookie_name.clone(),
            secure: v3.refresh_cookie_secure,
            same_site: v3.refresh_cookie_same_site.clone(),
        },
        email: Arc::new(SendgridEmail::new(
            config.sendgrid.api_key.clone(),
            config.sendgrid.sender_address.clone(),
        )),
        reset_link_base: v3.reset_link_base.clone(),
        access_decoding_key: Arc::new(config.web.auth.get_jwt_decoding_key().unwrap()),
        access_iss: v3.access_token_iss.clone(),
    };

    // credentials 付き CORS(`*` は使えないため origin は許可リスト)。
    let cors_origins: Vec<http::HeaderValue> = v3
        .cors_allowed_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect();
    let auth_cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::list(cors_origins))
        .allow_credentials(true)
        .allow_methods([http::Method::POST])
        .allow_headers([http::header::CONTENT_TYPE]);

    let (auth_v2_router, _auth_openapi) = routes::auth_v2::router().split_for_parts();
    let auth_v2_app = axum::Router::new()
        .nest("/auth/v2", auth_v2_router)
        .with_state(auth_v2_state)
        .layer(auth_cors);

    // ===== legacy(/api + 静的配信)。legacy /auth は auth_v2 へ移行済み(admin OIDC は未移行 TODO) =====
    let discord = Discord::new(&config.discord.approval_request_url);
    let legacy = init_routes(
        &config.web,
        config.sendgrid,
        oidc_client,
        s3_client,
        config.s3.bucket.clone(),
        discord,
        config.secrets,
    );

    let app = legacy.merge(auth_v2_app);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        &config.web.server.host, &config.web.server.port
    ))
    .await
    .unwrap();
    tracing::debug!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

#[instrument(skip(client_secret))]
async fn init_oidc(
    client_id: String,
    client_secret: String,
    issuer_url: String,
    redirect_url: String,
) -> OIDCClient {
    let http_client = reqwest::Client::new();

    let provider_metadata: CoreProviderMetadata =
        CoreProviderMetadata::discover_async(IssuerUrl::new(issuer_url).unwrap(), &http_client)
            .await
            .unwrap();

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    client
}

// TODO(sqlx移行): DB接続初期化(Migrator実行含む)は application 層(sqlx)へ再配線する
// #[instrument(skip(db))]
// pub async fn init_db(db: &Db) -> Result<DatabaseConnection, DbErr> {
//     debug!("Initializing database connection");
//     let db_conn = Database::connect(&db.address).await?;
//     Migrator::up(&db_conn, None).await?;
//     Ok(db_conn)
// }

pub fn init_logging(logging: Logging) {
    if logging.json {
        tracing_subscriber::registry()
            .with(logging.log_level.to_level_filter())
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(logging.log_level.to_level_filter())
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}

pub async fn init_s3(
    access_key_id: impl Into<String>,
    secret_access_key: impl Into<String>,
    endpoint: impl Into<String>,
) -> aws_sdk_s3::Client {
    let shared_cfg = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(
            Credentials::builder()
                .access_key_id(access_key_id)
                .secret_access_key(secret_access_key)
                .provider_name("default-provider")
                .build(),
        )
        .endpoint_url(endpoint)
        .region("ap-northeast-1")
        .load()
        .await;

    let s3_cfg = aws_sdk_s3::config::Builder::from(&shared_cfg)
        .force_path_style(true)
        .build();

    aws_sdk_s3::Client::from_conf(s3_cfg)
}
