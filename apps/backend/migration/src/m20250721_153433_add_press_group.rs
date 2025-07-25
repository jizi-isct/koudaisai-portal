use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();

        // group_typeの追加
        connection
            .execute_unprepared("CREATE TYPE group_type AS ENUM ('plan', 'press');")
            .await?;

        // groupにtypeカラムを追加
        connection
            .execute_unprepared(
                r#"ALTER TABLE "group" ADD COLUMN type group_type NOT NULL DEFAULT 'plan';"#,
            )
            .await?;

        // group_pressテーブルを追加
        connection
            .execute_unprepared(
                r#"CREATE TABLE group_press(
                id char(5) PRIMARY KEY REFERENCES "group"(id) DEFERRABLE INITIALLY DEFERRED,
                representative uuid NOT NULL REFERENCES users DEFERRABLE INITIALLY DEFERRED
            );"#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();

        // group_pressテーブルの削除
        connection
            .execute_unprepared("DROP TABLE IF EXISTS group_press;")
            .await?;

        // groupからtypeカラムを削除
        connection
            .execute_unprepared(r#"ALTER TABLE "group" DROP COLUMN IF EXISTS type;"#)
            .await?;

        // group_typeの削除
        connection
            .execute_unprepared("DROP TYPE IF EXISTS group_type;")
            .await?;

        Ok(())
    }
}
