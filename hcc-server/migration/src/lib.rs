pub use sea_schema::migration::*;

mod m01_000001_create_user_table;
mod m01_000002_create_user_index;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m01_000001_create_user_table::Migration),
            Box::new(m01_000002_create_user_index::Migration),
        ]
    }
}
