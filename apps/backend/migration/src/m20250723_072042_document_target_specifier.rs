use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();

        // Rename the column from `required_one_of_scopes` to `targets`
        connection
            .execute_unprepared(
                "ALTER TABLE document RENAME COLUMN required_one_of_scopes TO targets;",
            )
            .await?;

        // データの変換: 'none' を 'user/nologin' に置き換え、他の値を 'group/type/plan_' プレフィックス付きで変換
        connection
            .execute_unprepared(
                r#"
            UPDATE document
            SET targets = (
                SELECT array_agg(
                    CASE
                        WHEN val = 'none' THEN 'user/nologin'
                        ELSE 'group/type/plan_' || val
                    END
                    ORDER BY ordinality
                )
                FROM unnest(targets) WITH ORDINALITY AS t(val, ordinality)
            );
        "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();

        // Rename the column back from `targets` to `required_one_of_scopes`
        connection
            .execute_unprepared(
                "ALTER TABLE document RENAME COLUMN targets TO required_one_of_scopes;",
            )
            .await?;

        // データの変換: 'user/nologin' を 'none' に置き換え、他の値を 'group/type/plan_' プレフィックスを削除
        connection.execute_unprepared(r#"
            UPDATE document
            SET targets = (
                SELECT array_agg(new_val ORDER BY ordinality)
                FROM (
                    SELECT
                        CASE
                            WHEN val = 'user/nologin' THEN 'none'
                            WHEN val LIKE 'group/type/plan_%' THEN substring(val from 'group/type/plan_(.*)')
                            ELSE NULL
                        END AS new_val,
                        ordinality
                    FROM unnest(targets) WITH ORDINALITY AS t(val, ordinality)
                ) sub
                WHERE new_val IS NOT NULL
            );
        "#).await?;

        Ok(())
    }
}
