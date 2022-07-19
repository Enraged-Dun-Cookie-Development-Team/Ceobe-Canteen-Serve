mod migrations;

pub use mongo_models;
use mongo_connection::{MigratorTrait, MigrationTrait};
pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrators(&self) -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(migrations::mansion::Migration)]
    }
}
