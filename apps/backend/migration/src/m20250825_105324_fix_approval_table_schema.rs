use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let conn = manager.get_connection();

        conn.execute_unprepared(
            r#"ALTER TABLE approval_request_type_edit_exhibition_info
            DROP COLUMN is_child_friendly,
            DROP COLUMN exhibition_name;"#,
        )
        .await?;

        conn.execute_unprepared(
            r#"ALTER TABLE approval_request
            ADD COLUMN issue_reason text NOT NULL DEFAULT '',
            ADD COLUMN approval_reason text;"#,
        )
        .await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // Reverse the changes made in up():
        // 1. Remove the columns that were added to approval_request table
        conn.execute_unprepared(
            r#"ALTER TABLE approval_request
            DROP COLUMN issue_reason,
            DROP COLUMN approval_reason;"#,
        )
        .await?;

        // 2. Re-add the columns that were dropped from approval_request_type_edit_exhibition_info table
        conn.execute_unprepared(
            r#"ALTER TABLE approval_request_type_edit_exhibition_info
            ADD COLUMN is_child_friendly boolean,
            ADD COLUMN exhibition_name text;"#,
        )
        .await?;

        Ok(())
    }
}
