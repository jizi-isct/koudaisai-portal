pub use sea_orm_migration::prelude::*;

mod m20250203_100228_create_table_users;
mod m20250204_062103_create_table_exhibitors;
mod m20250214_132032_create_table_exhibitors_children;
mod m20250227_173624_create_table_forms;
mod m20250227_191549_create_table_form_responses;
mod m20250310_133355_create_table_revoked_refresh_tokens;
mod m20250425_085751_create_table_document;
mod m20250428_070428_add_column_required_one_of_scopes_to_table_document;
mod m20250430_064809_rename_column_file_url_of_document_format_pdf;
mod m20250501_161327_create_trigger_update_document_modtime;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250203_100228_create_table_users::Migration),
            Box::new(m20250204_062103_create_table_exhibitors::Migration),
            Box::new(m20250214_132032_create_table_exhibitors_children::Migration),
            Box::new(m20250227_173624_create_table_forms::Migration),
            Box::new(m20250227_191549_create_table_form_responses::Migration),
            Box::new(m20250310_133355_create_table_revoked_refresh_tokens::Migration),
            Box::new(m20250425_085751_create_table_document::Migration),
            Box::new(
                m20250428_070428_add_column_required_one_of_scopes_to_table_document::Migration,
            ),
            Box::new(m20250430_064809_rename_column_file_url_of_document_format_pdf::Migration),
            Box::new(m20250501_161327_create_trigger_update_document_modtime::Migration),
        ]
    }
}
