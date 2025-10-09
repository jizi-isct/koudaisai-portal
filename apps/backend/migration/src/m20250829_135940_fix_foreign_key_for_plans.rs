use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();

        // Drop existing foreign key constraints and recreate them as DEFERRABLE INITIALLY DEFERRED

        // group_plan table - modify foreign key to group table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan DROP CONSTRAINT IF EXISTS group_plan_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan ADD CONSTRAINT group_plan_id_fkey 
                   FOREIGN KEY (id) REFERENCES "group"(id) DEFERRABLE INITIALLY DEFERRED;"#,
            )
            .await?;

        // group_plan_booth table - modify foreign key to group_plan table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_booth DROP CONSTRAINT IF EXISTS group_plan_booth_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_booth ADD CONSTRAINT group_plan_booth_id_fkey 
                   FOREIGN KEY (id) REFERENCES group_plan(id) DEFERRABLE INITIALLY DEFERRED;"#,
            )
            .await?;

        // group_plan_general table - modify foreign key to group_plan table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_general DROP CONSTRAINT IF EXISTS group_plan_general_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_general ADD CONSTRAINT group_plan_general_id_fkey 
                   FOREIGN KEY (id) REFERENCES group_plan(id) DEFERRABLE INITIALLY DEFERRED;"#,
            )
            .await?;

        // group_plan_stage table - modify foreign key to group_plan table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_stage DROP CONSTRAINT IF EXISTS group_plan_stage_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_stage ADD CONSTRAINT group_plan_stage_id_fkey 
                   FOREIGN KEY (id) REFERENCES group_plan(id) DEFERRABLE INITIALLY DEFERRED;"#,
            )
            .await?;

        // group_plan_labo table - modify foreign key to group_plan table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_labo DROP CONSTRAINT IF EXISTS group_plan_labo_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_labo ADD CONSTRAINT group_plan_labo_id_fkey 
                   FOREIGN KEY (id) REFERENCES group_plan(id) DEFERRABLE INITIALLY DEFERRED;"#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();

        // Revert foreign key constraints back to non-deferrable

        // group_plan table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan DROP CONSTRAINT IF EXISTS group_plan_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan ADD CONSTRAINT group_plan_id_fkey 
                   FOREIGN KEY (id) REFERENCES "group"(id);"#,
            )
            .await?;

        // group_plan_booth table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_booth DROP CONSTRAINT IF EXISTS group_plan_booth_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_booth ADD CONSTRAINT group_plan_booth_id_fkey 
                   FOREIGN KEY (id) REFERENCES group_plan(id);"#,
            )
            .await?;

        // group_plan_general table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_general DROP CONSTRAINT IF EXISTS group_plan_general_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_general ADD CONSTRAINT group_plan_general_id_fkey 
                   FOREIGN KEY (id) REFERENCES group_plan(id);"#,
            )
            .await?;

        // group_plan_stage table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_stage DROP CONSTRAINT IF EXISTS group_plan_stage_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_stage ADD CONSTRAINT group_plan_stage_id_fkey 
                   FOREIGN KEY (id) REFERENCES group_plan(id);"#,
            )
            .await?;

        // group_plan_labo table
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_labo DROP CONSTRAINT IF EXISTS group_plan_labo_id_fkey;"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE group_plan_labo ADD CONSTRAINT group_plan_labo_id_fkey 
                   FOREIGN KEY (id) REFERENCES group_plan(id);"#,
            )
            .await?;

        Ok(())
    }
}
