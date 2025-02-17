use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::{debug, instrument};
use migration::{Migrator, MigratorTrait};
use crate::config::Db;

#[instrument(skip(db))]
pub async fn init_db(db: &Db) -> Result<DatabaseConnection, DbErr> {
    debug!("Initializing database connection");
    let db_conn = Database::connect(&db.address).await?;
    Migrator::up(&db_conn, None).await?;
    Ok(db_conn)
}