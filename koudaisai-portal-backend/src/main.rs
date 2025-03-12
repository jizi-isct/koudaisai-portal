use crate::config::{init_config, Db, Logging};
use crate::routes::init_routes;
use migration::{Migrator, MigratorTrait};
use openid::DiscoveredClient;
use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::{debug, info, instrument};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod config;
pub mod entities;
mod forms;
pub mod middlewares;
mod routes;
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
    let client_id = config.web.auth.keycloak.id.clone();
    let client_secret = config.web.auth.keycloak.secret.clone();
    let issuer = reqwest::Url::parse(&config.web.auth.keycloak.issuer).unwrap();
    let redirect = Some(format!(
        "{}{}",
        &config.web.server.host, "/auth/v1/admin/redirect"
    ));

    let oidc_client = DiscoveredClient::discover(client_id, client_secret, redirect, issuer)
        .await
        .unwrap();

    //app init
    let db = init_db(&config.db).await.unwrap();
    let app = init_routes(&config.web, db, oidc_client);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        &config.web.server.host, &config.web.server.port
    ))
    .await
    .unwrap();
    tracing::debug!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
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
