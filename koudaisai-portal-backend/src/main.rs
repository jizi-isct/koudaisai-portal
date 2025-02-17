use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
use tracing::info;
use tracing_core::LevelFilter;
use crate::config::init_config;
use crate::logging::init_logging;

pub mod entities;
pub mod config;
pub mod logging;
mod routes;

const MAJOR_VERSION: u32 = pkg_version_major!();
const MINOR_VERSION: u32 = pkg_version_minor!();
const PATCH_VERSION: u32 = pkg_version_patch!();
#[tokio::main]
async fn main() {
    //初期化
    let config = init_config().expect("TODO: panic message");
    init_logging(config.logging.log_level.to_level_filter());
    info!("Koudaisai Portal v{}.{}.{} (c) 2025 JIZI All Rights Reserved.", MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION);

    //axum

}
