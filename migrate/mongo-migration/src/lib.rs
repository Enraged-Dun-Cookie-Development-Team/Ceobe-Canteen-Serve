mod migrations;

use mongo_connection::{MigrationTrait, MigratorTrait};
pub use mongo_models;
pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrators(&self) -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(migrations::mansion::Migration)]
    }
}
