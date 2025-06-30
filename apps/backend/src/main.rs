use crate::config::{init_config, Db, Logging};
use crate::routes::init_routes;
use crate::util::oidc::OIDCClient;
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::Credentials;
use migration::{Migrator, MigratorTrait};
use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::{debug, info, instrument};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod config;
pub mod entities;
pub mod middlewares;
mod routes;
pub mod sea_orm_entities;
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

    //app init
    let db = init_db(&config.db).await.unwrap();
    let app = init_routes(
        &config.web,
        config.sendgrid,
        db,
        oidc_client,
        s3_client,
        config.s3.bucket.clone(),
    );

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        &config.web.server.host, &config.web.server.port
    ))
    .await
    .unwrap();
    tracing::debug!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
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

#[instrument(skip(db))]
pub async fn init_db(db: &Db) -> Result<DatabaseConnection, DbErr> {
    debug!("Initializing database connection");
    let db_conn = Database::connect(&db.address).await?;
    Migrator::up(&db_conn, None).await?;
    Ok(db_conn)
}

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
