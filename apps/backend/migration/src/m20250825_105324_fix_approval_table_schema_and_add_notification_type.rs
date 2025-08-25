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

        // Add APPROVAL_REQUEST to notification_type enum
        conn.execute_unprepared(r#"ALTER TYPE notification_type ADD VALUE 'APPROVAL_REQUEST';"#)
            .await?;

        // Create notification_type_approval_request table
        conn.execute_unprepared(
            r#"CREATE TABLE notification_type_approval_request (
            id uuid PRIMARY KEY REFERENCES notification(id) ON DELETE CASCADE,
            approval_request_id uuid NOT NULL REFERENCES approval_request(id) ON DELETE CASCADE
        );"#,
        )
        .await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // Reverse the changes made in up():
        // 1. Drop the notification_type_approval_request table
        conn.execute_unprepared(r#"DROP TABLE IF EXISTS notification_type_approval_request;"#)
            .await?;

        // 2. Remove APPROVAL_REQUEST from notification_type enum
        // Note: PostgreSQL doesn't support removing enum values directly,
        // so we recreate the enum without the APPROVAL_REQUEST value
        conn.execute_unprepared(r#"ALTER TYPE notification_type RENAME TO notification_type_old;"#)
            .await?;

        conn.execute_unprepared(r#"CREATE TYPE notification_type AS ENUM ('MARKDOWN');"#)
            .await?;

        conn.execute_unprepared(
            r#"ALTER TABLE notification ALTER COLUMN type TYPE notification_type USING type::text::notification_type;"#,
        )
            .await?;

        conn.execute_unprepared(r#"DROP TYPE notification_type_old;"#)
            .await?;

        // 3. Remove the columns that were added to approval_request table
        conn.execute_unprepared(
            r#"ALTER TABLE approval_request
            DROP COLUMN issue_reason,
            DROP COLUMN approval_reason;"#,
        )
        .await?;

        // 4. Re-add the columns that were dropped from approval_request_type_edit_exhibition_info table
        conn.execute_unprepared(
            r#"ALTER TABLE approval_request_type_edit_exhibition_info
            ADD COLUMN is_child_friendly boolean,
            ADD COLUMN exhibition_name text;"#,
        )
        .await?;

        Ok(())
    }
}
