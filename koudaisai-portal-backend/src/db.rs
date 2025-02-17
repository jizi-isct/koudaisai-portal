use crate::config::Db;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::{debug, instrument};

#[instrument(skip(db))]
pub async fn init_db(db: &Db) -> Result<DatabaseConnection, DbErr> {
    debug!("Initializing database connection");
    let db_conn = Database::connect(&db.address).await?;
    Migrator::up(&db_conn, None).await?;
    Ok(db_conn)
}
