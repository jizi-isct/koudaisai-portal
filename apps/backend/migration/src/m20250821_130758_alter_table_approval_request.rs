use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let conn = manager.get_connection();

        conn.execute_unprepared(
            r#"ALTER TABLE approval_request_type_edit_exhibition_info
            DROP COLUMN is_exhibition_name_explicit_null,
            DROP COLUMN is_icon_id_explicit_null,
            DROP COLUMN is_description_explicit_null,
            ADD COLUMN is_child_friendly boolean;"#,
        )
        .await?;

        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared(
            r#"ALTER TABLE approval_request_type_edit_exhibition_info
            DROP COLUMN is_child_friendly,
            ADD COLUMN is_exhibition_name_explicit_null boolean NOT NULL DEFAULT false,
            ADD COLUMN is_icon_id_explicit_null boolean NOT NULL DEFAULT false,
            ADD COLUMN is_description_explicit_null boolean NOT NULL DEFAULT false;"#,
        )
        .await?;

        Ok(())
    }
}
