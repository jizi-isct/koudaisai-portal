pub use sea_orm_migration::prelude::*;

mod m20250203_100228_create_table_users;
mod m20250204_062103_create_table_exhibitors;
mod m20250214_132032_create_table_exhibitors_children;
mod m20250227_173624_create_table_forms;
mod m20250227_191549_create_table_form_responses;

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
        ]
    }
}
