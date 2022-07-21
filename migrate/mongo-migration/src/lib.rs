mod migrations;

use mongo_migrate_util::{MigratorTrait, MigrationTrait};
pub use mongo_models;
pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrators(&self) -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(migrations::mansion::Migration)]
    }
}
