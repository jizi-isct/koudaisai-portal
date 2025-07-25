use crate::sea_orm::Statement;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();
        // テーブルの削除
        connection
            .execute_unprepared("DROP TABLE IF EXISTS exhibitors_category_booth;")
            .await?;
        connection
            .execute_unprepared("DROP TABLE IF EXISTS exhibitors_category_general;")
            .await?;
        connection
            .execute_unprepared("DROP TABLE IF EXISTS exhibitors_category_stage;")
            .await?;
        connection
            .execute_unprepared("DROP TABLE IF EXISTS exhibitors_category_labo;")
            .await?;

        // Rename table and types
        connection
            .execute_unprepared(r#"ALTER TABLE exhibitors_root RENAME TO "group";"#)
            .await?;
        connection
            .execute_unprepared("ALTER TYPE exhibition_type RENAME TO plan_type;")
            .await?;

        // groupテーブルの分割処理
        connection
            .execute_unprepared(
                r#"CREATE TABLE group_plan (
                    id TEXT PRIMARY KEY REFERENCES "group"(id),
                    type plan_type NOT NULL
                );"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"CREATE TABLE group_plan_booth (
                    id TEXT PRIMARY KEY REFERENCES group_plan(id),
                    representative1 UUID NOT NULL REFERENCES users(id),
                    representative2 UUID NOT NULL REFERENCES users(id),
                    representative3 UUID NOT NULL REFERENCES users(id)
                );"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"CREATE TABLE group_plan_general (
                    id TEXT PRIMARY KEY REFERENCES group_plan(id),
                    representative1 UUID NOT NULL REFERENCES users(id),
                    representative2 UUID NOT NULL REFERENCES users(id),
                    representative3 UUID NOT NULL REFERENCES users(id)
                );"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"CREATE TABLE group_plan_stage (
                    id TEXT PRIMARY KEY REFERENCES group_plan(id),
                    representative1 UUID NOT NULL REFERENCES users(id),
                    representative2 UUID NOT NULL REFERENCES users(id),
                    representative3 UUID NOT NULL REFERENCES users(id)
                );"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"CREATE TABLE group_plan_labo (
                    id TEXT PRIMARY KEY REFERENCES group_plan(id),
                    representative UUID NOT NULL REFERENCES users(id)
                );"#,
            )
            .await?;

        // データ移行
        connection
            .execute_unprepared(
                r#"INSERT INTO group_plan (id, type) SELECT id, type FROM "group";"#,
            )
            .await?;
        connection.execute_unprepared(
            r#"INSERT INTO group_plan_booth (id, representative1, representative2, representative3) SELECT id, representative1, representative2, representative3 FROM "group" WHERE type = 'BOOTH';"#
        ).await?;
        connection.execute_unprepared(
            r#"INSERT INTO group_plan_general (id, representative1, representative2, representative3) SELECT id, representative1, representative2, representative3 FROM "group" WHERE type = 'GENERAL';"#
        ).await?;
        connection.execute_unprepared(
            r#"INSERT INTO group_plan_stage (id, representative1, representative2, representative3) SELECT id, representative1, representative2, representative3 FROM "group" WHERE type = 'STAGE';"#
        ).await?;
        connection.execute_unprepared(
            r#"INSERT INTO group_plan_labo (id, representative) SELECT id, representative1 FROM "group" WHERE type = 'LABO';"#
        ).await?;

        // ダミーユーザーの削除
        connection
            .execute_unprepared(
                r#"ALTER TABLE "group" DROP CONSTRAINT IF EXISTS exhibitors_root_representative2_fkey"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"ALTER TABLE "group" DROP CONSTRAINT IF EXISTS exhibitors_root_representative3_fkey"#,
            )
            .await?;
        connection
            .execute_unprepared(
                r#"DELETE FROM users u WHERE EXISTS (
                SELECT 1 FROM "group" g
                JOIN group_plan gp ON gp.id = g.id
                WHERE gp.type = 'LABO'
                AND (u.id = g.representative2 OR u.id = g.representative3)
            );"#,
            )
            .await?;

        // テーブル構造
        connection.execute_unprepared(
            r#"ALTER TABLE "group" DROP COLUMN type, DROP COLUMN exhibition_name, DROP COLUMN icon_id, DROP COLUMN description, DROP COLUMN representative1, DROP COLUMN representative2, DROP COLUMN representative3;"#
        ).await?;
        connection
            .execute_unprepared("DROP TYPE stage_type;")
            .await?;

        //usersの列名変更
        connection
            .execute_unprepared("ALTER TABLE users RENAME COLUMN exhibition_id TO group_id;")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();

        //usersの列名変更
        connection
            .execute_unprepared("ALTER TABLE users RENAME COLUMN group_id TO exhibition_id;")
            .await?;

        // Restore columns to "group" table
        connection
            .execute_unprepared(
                r#"ALTER TABLE "group" ADD COLUMN type plan_type, 
                ADD COLUMN exhibition_name text, 
                ADD COLUMN icon_id text, 
                ADD COLUMN description text, 
                ADD COLUMN representative1 uuid REFERENCES users(id) DEFERRABLE INITIALLY DEFERRED, 
                ADD COLUMN representative2 uuid REFERENCES users(id) DEFERRABLE INITIALLY DEFERRED,
                ADD COLUMN representative3 uuid REFERENCES users(id) DEFERRABLE INITIALLY DEFERRED;"#,
            )
            .await?;

        // Create dummy users for representative2 and representative3 from group_plan_labo
        connection.execute_unprepared(
            "ALTER TABLE group_plan_labo ADD COLUMN representative2 uuid DEFAULT gen_random_uuid() NOT NULL;"
        ).await?;
        connection.execute_unprepared(
            "ALTER TABLE group_plan_labo ADD COLUMN representative3 uuid DEFAULT gen_random_uuid() NOT NULL;"
        ).await?;
        connection
            .execute_unprepared(
                r#"WITH labo_representatives AS (
                    SELECT gpl.id, gpl.representative2, gpl.representative3
                    FROM group_plan_labo gpl
                    JOIN group_plan gp ON gp.id = gpl.id
                    WHERE gp.type = 'LABO'
                )
                INSERT INTO users (id, m_address, password_salt, exhibition_id, name, password_updated_at)
                SELECT
                    representative2,
                    'dummy.' || id || '.rep2@m.isct.ac.jp',
                    md5(random()::text),
                    id,
                    'Dummy Representative 2 for ' || id,
                    '1970-01-01 00:00:00+00'::timestamp with time zone
                FROM labo_representatives
                WHERE NOT EXISTS (SELECT 1 FROM users WHERE id = representative2);"#,
            )
            .await?;

        connection
            .execute_unprepared(
                r#"WITH labo_representatives AS (
                    SELECT gpl.id, gpl.representative2, gpl.representative3
                    FROM group_plan_labo gpl
                    JOIN group_plan gp ON gp.id = gpl.id
                    WHERE gp.type = 'LABO'
                )
                INSERT INTO users (id, m_address, password_salt, exhibition_id, name, password_updated_at)
                SELECT
                    representative3,
                    'dummy.' || id || '.rep3@m.isct.ac.jp',
                    md5(random()::text),
                    id,
                    'Dummy Representative 3 for ' || id,
                    '1970-01-01 00:00:00+00'::timestamp with time zone
                FROM labo_representatives
                WHERE NOT EXISTS (SELECT 1 FROM users WHERE id = representative3);"#,
            )
            .await?;

        // Migrate data back from group_plan_* tables to "group" table
        connection
            .execute_unprepared(
                r#"UPDATE "group" g
                SET type = gp.type
                FROM group_plan gp
                WHERE g.id = gp.id;"#,
            )
            .await?;

        // Update representative fields from group_plan_booth
        connection
            .execute_unprepared(
                r#"UPDATE "group" g
                SET representative1 = gpb.representative1,
                    representative2 = gpb.representative2,
                    representative3 = gpb.representative3
                FROM group_plan_booth gpb
                WHERE g.id = gpb.id;"#,
            )
            .await?;

        // Update representative fields from group_plan_general
        connection
            .execute_unprepared(
                r#"UPDATE "group" g
                SET representative1 = gpg.representative1,
                    representative2 = gpg.representative2,
                    representative3 = gpg.representative3
                FROM group_plan_general gpg
                WHERE g.id = gpg.id;"#,
            )
            .await?;

        // Update representative fields from group_plan_stage
        connection
            .execute_unprepared(
                r#"UPDATE "group" g
                SET representative1 = gps.representative1,
                    representative2 = gps.representative2,
                    representative3 = gps.representative3
                FROM group_plan_stage gps
                WHERE g.id = gps.id;"#,
            )
            .await?;

        // Update representative fields from group_plan_labo
        connection
            .execute_unprepared(
                r#"UPDATE "group" g
                SET representative1 = gpl.representative,
                    representative2 = gpl.representative2,
                    representative3 = gpl.representative3
                FROM group_plan_labo gpl
                WHERE g.id = gpl.id;"#,
            )
            .await?;

        // Recreate exhibitors_category_* tables
        connection
            .execute_unprepared(
                r#"CREATE TABLE exhibitors_category_booth(
                    id char(5) PRIMARY KEY NOT NULL REFERENCES "group" DEFERRABLE INITIALLY DEFERRED,
                    location text,
                    starting_time_day1 timestamp with time zone,
                    ending_time_day1 timestamp with time zone,
                    starting_time_day2 timestamp with time zone,
                    ending_time_day2 timestamp with time zone
                );"#,
            )
            .await?;

        connection
            .execute_unprepared(
                r#"CREATE TABLE exhibitors_category_general(
                    id char(5) PRIMARY KEY NOT NULL REFERENCES "group" DEFERRABLE INITIALLY DEFERRED,
                    location text,
                    starting_time_day1 timestamp with time zone,
                    ending_time_day1 timestamp with time zone,
                    starting_time_day2 timestamp with time zone,
                    ending_time_day2 timestamp with time zone
                );"#,
            )
            .await?;

        connection
            .execute_unprepared(
                r#"CREATE TYPE stage_type AS ENUM ('OUTDOOR', 'AUDIOTORIUM', 'WOOD_DECK', 'TAKIPLAZA', 'HALL');"#,
            )
            .await?;

        connection
            .execute_unprepared(
                r#"CREATE TABLE exhibitors_category_stage(
                    id char(5) PRIMARY KEY NOT NULL REFERENCES "group" DEFERRABLE INITIALLY DEFERRED,
                    type stage_type
                );"#,
            )
            .await?;

        connection
            .execute_unprepared(
                r#"CREATE TABLE exhibitors_category_labo(
                    id char(5) PRIMARY KEY NOT NULL REFERENCES "group" DEFERRABLE INITIALLY DEFERRED,
                    location text,
                    starting_time_day1 timestamp with time zone,
                    ending_time_day1 timestamp with time zone,
                    starting_time_day2 timestamp with time zone,
                    ending_time_day2 timestamp with time zone
                );"#,
            )
            .await?;

        // Insert data into exhibitors_category_* tables from group_plan_* tables
        // For booth
        connection
            .execute_unprepared(
                r#"INSERT INTO exhibitors_category_booth (id)
                SELECT id FROM group_plan_booth;"#,
            )
            .await?;

        // For general
        connection
            .execute_unprepared(
                r#"INSERT INTO exhibitors_category_general (id)
                SELECT id FROM group_plan_general;"#,
            )
            .await?;

        // For stage
        connection
            .execute_unprepared(
                r#"INSERT INTO exhibitors_category_stage (id)
                SELECT id FROM group_plan_stage;"#,
            )
            .await?;

        // For labo
        connection
            .execute_unprepared(
                r#"INSERT INTO exhibitors_category_labo (id)
                SELECT id FROM group_plan_labo;"#,
            )
            .await?;

        // Drop group_plan_* tables
        connection
            .execute_unprepared("DROP TABLE IF EXISTS group_plan_booth;")
            .await?;
        connection
            .execute_unprepared("DROP TABLE IF EXISTS group_plan_general;")
            .await?;
        connection
            .execute_unprepared("DROP TABLE IF EXISTS group_plan_stage;")
            .await?;
        connection
            .execute_unprepared("DROP TABLE IF EXISTS group_plan_labo;")
            .await?;
        connection
            .execute_unprepared("DROP TABLE IF EXISTS group_plan;")
            .await?;

        // Rename "group" table back to exhibitors_root
        connection
            .execute_unprepared(r#"ALTER TABLE "group" RENAME TO exhibitors_root;"#)
            .await?;

        // Rename plan_type back to exhibition_type
        connection
            .execute_unprepared("ALTER TYPE plan_type RENAME TO exhibition_type;")
            .await?;

        Ok(())
    }
}
