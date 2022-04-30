mod migrations;
pub mod utils;

pub use mongo_models;
use utils::migrator::MigratorTrait;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrators(&self) -> Vec<Box<dyn utils::migration::MigrationTrait>> {
        vec![Box::new(migrations::mansion::Migration)]
    }
}
