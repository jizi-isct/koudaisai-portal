use crate::sea_orm::Statement;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create approval_request_type enum
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TYPE approval_request_type AS ENUM ('type_edit_exhibition_info');
                "#
                .trim(),
            ))
            .await?;

        // Create approval_request_status enum
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TYPE approval_request_status AS ENUM ('pending', 'approved', 'rejected', 'closed');
                "#
                    .trim(),
            ))
            .await?;

        // Create approval_request table with type column
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TABLE approval_request(
                    id uuid PRIMARY KEY,
                    issued_at timestamp with time zone NOT NULL,
                    issued_by uuid NOT NULL REFERENCES users,
                    type approval_request_type NOT NULL,
                    status approval_request_status NOT NULL,
                    approved_by uuid
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
                CREATE TABLE approval_request_type_edit_exhibition_info(
                    id uuid PRIMARY KEY REFERENCES approval_request(id) ON DELETE CASCADE DEFERRABLE,
                    exhibition_name text,
                    icon_id text,
                    description text,
                    is_exhibition_name_explicit_null boolean NOT NULL,
                    is_icon_id_explicit_null boolean NOT NULL,
                    is_description_explicit_null boolean NOT NULL
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
                DROP TABLE IF EXISTS approval_request_type_edit_exhibition_info;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE IF EXISTS approval_request;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TYPE IF EXISTS approval_request_status;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TYPE IF EXISTS approval_request_type;
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }
}
