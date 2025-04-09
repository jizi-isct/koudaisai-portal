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
                CREATE TABLE exhibitors_category_booth(
                    id char(5) PRIMARY KEY NOT NULL REFERENCES exhibitors_root DEFERRABLE INITIALLY DEFERRED,
                    location text,
                    starting_time_day1 timestamp with time zone,
                    ending_time_day1 timestamp with time zone,
                    starting_time_day2 timestamp with time zone,
                    ending_time_day2 timestamp with time zone
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
                CREATE TABLE exhibitors_category_general(
                    id char(5) PRIMARY KEY NOT NULL REFERENCES exhibitors_root DEFERRABLE INITIALLY DEFERRED,
                    location text,
                    starting_time_day1 timestamp with time zone,
                    ending_time_day1 timestamp with time zone,
                    starting_time_day2 timestamp with time zone,
                    ending_time_day2 timestamp with time zone
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
                CREATE TYPE stage_type AS ENUM ('OUTDOOR', 'AUDIOTORIUM', 'WOOD_DECK', 'TAKIPLAZA', 'HALL');
                "#
                    .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TABLE exhibitors_category_stage(
                    id char(5) PRIMARY KEY NOT NULL REFERENCES exhibitors_root DEFERRABLE INITIALLY DEFERRED,
                    type stage_type
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
                CREATE TABLE exhibitors_category_labo(
                    id char(5) PRIMARY KEY NOT NULL REFERENCES exhibitors_root DEFERRABLE INITIALLY DEFERRED,
                    location text,
                    starting_time_day1 timestamp with time zone,
                    ending_time_day1 timestamp with time zone,
                    starting_time_day2 timestamp with time zone,
                    ending_time_day2 timestamp with time zone
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
                DROP TABLE exhibitors_category_labo;
                "#
                .trim(),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE exhibitors_category_stage;
                "#
                .trim(),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TYPE stage_type;
                "#
                .trim(),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE exhibitors_category_general;
                "#
                .trim(),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE exhibitors_category_booth;
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }
}
