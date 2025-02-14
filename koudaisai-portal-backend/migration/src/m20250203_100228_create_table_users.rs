use crate::sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE users(
                id uuid PRIMARY KEY,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                m_address TEXT CHECK(m_address ~ '^[a-zA-Z0-9_+-]+\.[a-zA-Z0-9_+-]+\.[0-9][0-9][0-9][0-9]@m\.isct\.ac\.jp') NOT NULL,
                password_hash TEXT NOT NULL
            );
            "#.trim(),
        )).await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE OR REPLACE FUNCTION update_timestamp()
                    RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = NOW();
                    RETURN NEW;
                END;
                $$ language 'plpgsql';
                "#.trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TRIGGER update_users_modtime
                    BEFORE UPDATE ON users
                    FOR EACH ROW
                    EXECUTE PROCEDURE update_timestamp();
                "#.trim(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"DROP TABLE users;"#,
            ))
            .await?;

        Ok(())
    }
}
