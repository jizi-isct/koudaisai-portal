use crate::config::init_config;
use crate::db::init_db;
use crate::logging::init_logging;
use crate::routes::init_routes;
use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
use tracing::info;

pub mod config;
mod db;
pub mod entities;
pub mod logging;
mod routes;
pub mod util;

const MAJOR_VERSION: u32 = pkg_version_major!();
const MINOR_VERSION: u32 = pkg_version_minor!();
const PATCH_VERSION: u32 = pkg_version_patch!();
#[tokio::main]
async fn main() {
    //初期化
    let config = init_config().unwrap();
    init_logging(config.logging.log_level.to_level_filter());
    info!(
        "Koudaisai Portal v{}.{}.{} (c) 2025 JIZI All Rights Reserved.",
        MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION
    );

    //app init
    let db = init_db(&config.db).await.unwrap();
    let app = init_routes(&config.web, db);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.web.server.host, config.web.server.port
    ))
    .await
    .unwrap();
    tracing::debug!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
