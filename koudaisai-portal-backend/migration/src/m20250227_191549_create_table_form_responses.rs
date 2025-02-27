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
                CREATE TABLE form_responses(
                    response_id uuid PRIMARY KEY,
                    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    form_id uuid NOT NULL REFERENCES forms,
                    respondent_id uuid NOT NULL REFERENCES users,
                    answers json NOT NULL
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
                CREATE TRIGGER form_responses_modtime
                    BEFORE UPDATE ON form_responses
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
                DROP TABLE form_responses;
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }
}
