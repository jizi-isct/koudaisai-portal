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
            CREATE TABLE document_category(
                id uuid PRIMARY KEY,
                created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                title TEXT NOT NULL
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
                CREATE TYPE document_format AS ENUM ('PDF', 'MARKDOWN');
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
            CREATE TABLE document(
                id uuid PRIMARY KEY,
                created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                created_by uuid NOT NULL,
                updated_by uuid NOT NULL,
                title TEXT NOT NULL,
                format document_format NOT NULL,
                category uuid REFERENCES document_category(id) ON DELETE SET NULL
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
            CREATE TABLE document_format_pdf(
                id uuid PRIMARY KEY REFERENCES document(id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED,
                file_url TEXT NOT NULL
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
            CREATE TABLE document_format_markdown(
                id uuid PRIMARY KEY REFERENCES document(id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED,
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
            DROP TABLE document_format_markdown;
            "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
            DROP TABLE document_format_pdf;
            "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
            DROP TABLE document;
            "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
            DROP TYPE document_format;
            "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
            DROP TABLE document_category;
            "#
                .trim(),
            ))
            .await?;

        Ok(())
    }
}
