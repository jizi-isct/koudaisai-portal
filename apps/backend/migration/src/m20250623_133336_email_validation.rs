use crate::sea_orm::Statement;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                ALTER TABLE users DROP CONSTRAINT users_m_address_check;
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                ALTER TABLE users ADD CONSTRAINT users_m_address_check CHECK (m_address ~ '^[a-zA-Z0-9_+-]+\.[a-zA-Z0-9_+-]+\.[0-9][0-9][0-9][0-9]@m\.isct\.ac\.jp');
                "#
                    .trim(),
            ))
            .await?;

        Ok(())
    }
}
