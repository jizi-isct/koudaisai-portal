use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::{debug, instrument};
use crate::config::Db;

#[instrument(skip(db))]
pub async fn init_db(db: &Db) -> Result<DatabaseConnection, DbErr> {
    debug!("Initializing database connection");
    Database::connect(&db.address).await
}