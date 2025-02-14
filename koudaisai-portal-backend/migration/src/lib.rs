pub use sea_orm_migration::prelude::*;

mod m20250203_100228_create_table_users;
mod m20250204_062103_create_table_exhibitors;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250203_100228_create_table_users::Migration),
            Box::new(m20250204_062103_create_table_exhibitors::Migration),
        ]
    }
}
