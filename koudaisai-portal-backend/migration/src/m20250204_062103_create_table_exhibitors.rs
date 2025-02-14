use crate::sea_orm::Statement;
use sea_orm_migration::prelude::*;

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
                CREATE TYPE exhibition_type AS ENUM ('BOOTH', 'GENERAL', 'STAGE', 'LABO');
                "#
                .trim(),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TABLE exhibitors_root(
                    id char(5) PRIMARY KEY,
                    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    exhibitor_name text NOT NULL,
                    type exhibition_type NOT NULL,
                    exhibition_name text,
                    icon_id text,
                    description text,
                    representative1 uuid REFERENCES users,
                    representative2 uuid REFERENCES users,
                    representative3 uuid REFERENCES users
                );
                "#
                .trim(),
            ))
            .await?;
        manager.get_connection().execute(
            Statement::from_string(
                manager.get_database_backend(),
                r#"
                ALTER TABLE users ADD COLUMN exhibition_id text NOT NULL REFERENCES exhibitors_root DEFERRABLE INITIALLY DEFERRED;
                "#.trim(),
            )
        ).await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TRIGGER update_exhibitors_root_modtime
                    BEFORE UPDATE ON exhibitors_root
                    FOR EACH ROW
                    EXECUTE PROCEDURE update_timestamp();
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
                ALTER TABLE users DROP COLUMN exhibition_id;
                "#
                .trim(),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE exhibitors_root;
                "#
                .trim(),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TYPE exhibition_type;
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }
}
