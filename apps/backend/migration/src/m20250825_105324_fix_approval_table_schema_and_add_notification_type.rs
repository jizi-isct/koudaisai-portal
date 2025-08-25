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

        // Move title from notification table to notification_type_markdown table
        conn.execute_unprepared(
            r#"ALTER TABLE notification_type_markdown ADD COLUMN title text NOT NULL DEFAULT '';"#,
        )
        .await?;

        // Migrate existing title data from notification to notification_type_markdown
        conn.execute_unprepared(
            r#"UPDATE notification_type_markdown 
               SET title = notification.title 
               FROM notification 
               WHERE notification_type_markdown.id = notification.id 
               AND notification.type = 'MARKDOWN';"#,
        )
        .await?;

        conn.execute_unprepared(r#"ALTER TABLE notification DROP COLUMN title;"#)
            .await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // Reverse the changes made in up():
        // 1. Restore title column to notification table and remove from notification_type_markdown
        conn.execute_unprepared(
            r#"ALTER TABLE notification ADD COLUMN title text NOT NULL DEFAULT '';"#,
        )
        .await?;

        // Migrate title data back from notification_type_markdown to notification
        conn.execute_unprepared(
            r#"UPDATE notification 
               SET title = notification_type_markdown.title 
               FROM notification_type_markdown 
               WHERE notification.id = notification_type_markdown.id 
               AND notification.type = 'MARKDOWN';"#,
        )
        .await?;

        conn.execute_unprepared(r#"ALTER TABLE notification_type_markdown DROP COLUMN title;"#)
            .await?;

        // 2. Drop the notification_type_approval_request table
        conn.execute_unprepared(r#"DROP TABLE IF EXISTS notification_type_approval_request;"#)
            .await?;

        // 3. Remove APPROVAL_REQUEST from notification_type enum
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

        // 4. Remove the columns that were added to approval_request table
        conn.execute_unprepared(
            r#"ALTER TABLE approval_request
            DROP COLUMN issue_reason,
            DROP COLUMN approval_reason;"#,
        )
        .await?;

        // 5. Re-add the columns that were dropped from approval_request_type_edit_exhibition_info table
        conn.execute_unprepared(
            r#"ALTER TABLE approval_request_type_edit_exhibition_info
            ADD COLUMN is_child_friendly boolean,
            ADD COLUMN exhibition_name text;"#,
        )
        .await?;

        Ok(())
    }
}
