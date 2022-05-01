use std::any::TypeId;

use mongodb::{Collection, Database};

use super::{manager::Manager, migration::MigrationTrait};

#[async_trait::async_trait]
pub trait MigratorTrait {
    fn migrators(&self) -> Vec<Box<dyn MigrationTrait>>;

    async fn register<D: DbManager + Send + 'static>(
        &self, db: D,
    ) -> Result<D, mongodb::error::Error> {
        let manager = Manager::builder().db(db.get_db()).build();
        for migrate in self.migrators() {
            let _ = migrate.migrate(&manager).await?;
        }
        let Manager { db, collections } = manager;
        Ok(<D as DbManager>::from_collects(db, collections))
    }
}

pub trait DbManager {
    fn get_db(self) -> Database;
    fn from_collects<I: IntoIterator<Item = (TypeId, Collection<()>)>>(
        db: Database, iter: I,
    ) -> Self;
}
