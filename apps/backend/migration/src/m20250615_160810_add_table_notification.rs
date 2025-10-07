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
                CREATE TYPE notification_type AS ENUM ('MARKDOWN');
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
            CREATE TABLE notification(
                id uuid PRIMARY KEY,
                created_at timestamp with time zone NOT NULL,
                updated_at timestamp with time zone NOT NULL,
                created_by uuid,
                updated_by uuid,
                title TEXT NOT NULL,
                target TEXT[] NOT NULL,
                type notification_type NOT NULL
            );
            "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TABLE notification_type_markdown(
                    id uuid PRIMARY KEY REFERENCES notification(id) ON DELETE CASCADE DEFERRABLE,
                    content TEXT NOT NULL
                );
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
                DROP TABLE IF EXISTS notification_type_markdown;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE IF EXISTS notification;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TYPE IF EXISTS notification_type;
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }
}
