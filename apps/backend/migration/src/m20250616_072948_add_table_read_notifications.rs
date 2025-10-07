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
            CREATE TABLE read_notifications(
                id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id uuid REFERENCES users(id) ON DELETE CASCADE,
                notification_id uuid NOT NULL REFERENCES notification(id) ON DELETE CASCADE
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
            DROP TABLE read_notifications;
            "#,
            ))
            .await?;

        Ok(())
    }
}
