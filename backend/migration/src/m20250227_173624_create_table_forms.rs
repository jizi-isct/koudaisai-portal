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
                CREATE TABLE forms(
                    form_id uuid PRIMARY KEY,
                    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    info json NOT NULL,
                    items json NOT NULL,
                    access_control_roles text[] NOT NULL
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
                CREATE TRIGGER forms_modtime
                    BEFORE UPDATE ON forms
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
                DROP TABLE forms;
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }
}
