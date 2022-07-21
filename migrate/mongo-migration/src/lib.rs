mod migrations;

use mongo_migrate_util::{MigrationTrait, MigratorTrait};
pub use mongo_models::{self, mongo_connection};
pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrators(&self) -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(migrations::mansion::Migration)]
    }
}
