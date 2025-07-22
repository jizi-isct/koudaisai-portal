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
mod m20250509_065506_add_document_format_misc;
mod m20250509_072250_rename_column_file_url_of_document_format_misc;
mod m20250512_110316_add_column_file_name;
mod m20250513_161809_add_column_emoji_to_table_document_category;
mod m20250615_160810_add_table_notification;
mod m20250616_072948_add_table_read_notifications;
mod m20250623_045042_add_external_form;
mod m20250623_133336_email_validation;
mod m20250626_132332_password_updated_at;
mod m20250629_114156_change_column_name_name;
mod m20250702_085908_new_table_approval_request;
mod m20250718_075715_add_table_group;
mod m20250721_153433_add_press_group;

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
            Box::new(m20250509_065506_add_document_format_misc::Migration),
            Box::new(m20250509_072250_rename_column_file_url_of_document_format_misc::Migration),
            Box::new(m20250512_110316_add_column_file_name::Migration),
            Box::new(m20250513_161809_add_column_emoji_to_table_document_category::Migration),
            Box::new(m20250615_160810_add_table_notification::Migration),
            Box::new(m20250616_072948_add_table_read_notifications::Migration),
            Box::new(m20250623_045042_add_external_form::Migration),
            Box::new(m20250623_133336_email_validation::Migration),
            Box::new(m20250626_132332_password_updated_at::Migration),
            Box::new(m20250629_114156_change_column_name_name::Migration),
            Box::new(m20250702_085908_new_table_approval_request::Migration),
            Box::new(m20250718_075715_add_table_group::Migration),
            Box::new(m20250721_153433_add_press_group::Migration),
        ]
    }
}
