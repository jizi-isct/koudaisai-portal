use crate::sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add name column
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                ALTER TABLE users ADD COLUMN name TEXT;
                "#
                .trim(),
            ))
            .await?;

        // Populate name column with concatenated first_name and last_name
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                UPDATE users SET name = first_name || ' ' || last_name;
                "#
                .trim(),
            ))
            .await?;

        // Make name column NOT NULL
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                ALTER TABLE users ALTER COLUMN name SET NOT NULL;
                "#
                .trim(),
            ))
            .await?;

        // Drop first_name and last_name columns
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                ALTER TABLE users DROP COLUMN first_name, DROP COLUMN last_name;
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add first_name and last_name columns
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                ALTER TABLE users ADD COLUMN first_name TEXT, ADD COLUMN last_name TEXT;
                "#
                .trim(),
            ))
            .await?;

        // Split name into first_name and last_name
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                UPDATE users SET 
                    first_name = split_part(name, ' ', 1),
                    last_name = substring(name from position(' ' in name) + 1);
                "#
                .trim(),
            ))
            .await?;

        // Make first_name and last_name NOT NULL
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                ALTER TABLE users 
                    ALTER COLUMN first_name SET NOT NULL,
                    ALTER COLUMN last_name SET NOT NULL;
                "#
                .trim(),
            ))
            .await?;

        // Drop name column
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                ALTER TABLE users DROP COLUMN name;
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }
}
