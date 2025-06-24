use sea_orm_migration::sea_orm::Statement;
use sea_orm_migration::{prelude::*, schema::*};

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
                DROP TABLE form_responses;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE forms;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TYPE form_type AS ENUM ('BUILTIN', 'EXTERNAL');
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TABLE form(
                    id uuid PRIMARY KEY,
                    created_at timestamp with time zone NOT NULL,
                    updated_at timestamp with time zone NOT NULL,
                    created_by uuid,
                    updated_by uuid,
                    targets text[] NOT NULL,
                    form_name text NOT NULL,
                    summary text NOT NULL,
                    due_date timestamp with time zone,
                    type form_type NOT NULL
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
                CREATE TABLE form_type_builtin(
                    form_id uuid PRIMARY KEY REFERENCES form(id) ON DELETE CASCADE,
                    info json NOT NULL,
                    items json NOT NULL,
                    access_control_roles text[] NOT NULL
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
                CREATE TABLE form_type_builtin_response(
                    response_id uuid PRIMARY KEY,
                    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    form_id uuid NOT NULL REFERENCES form_type_builtin(form_id) ON DELETE CASCADE,
                    respondent_id uuid NOT NULL REFERENCES users,
                    answers json NOT NULL
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
                CREATE TABLE form_type_external(
                    form_id uuid PRIMARY KEY REFERENCES form(id) ON DELETE CASCADE,
                    form_url text NOT NULL
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
                DROP TABLE form_type_external;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE form_type_builtin_response;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE form_type_builtin;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TABLE form;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                DROP TYPE form_type;
                "#
                .trim(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE TABLE forms(
                    form_id uuid PRIMARY KEY,
                    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    info json NOT NULL,
                    items json NOT NULL,
                    access_control_roles text[] NOT NULL
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
                CREATE TABLE form_responses(
                    response_id uuid PRIMARY KEY,
                    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
                    form_id uuid NOT NULL REFERENCES forms,
                    respondent_id uuid NOT NULL REFERENCES users,
                    answers json NOT NULL
                );
                "#
                .trim(),
            ))
            .await?;

        Ok(())
    }
}
