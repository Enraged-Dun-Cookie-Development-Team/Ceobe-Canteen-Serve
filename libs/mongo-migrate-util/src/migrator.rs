use std::any::TypeId;

use mongodb::{Collection, Database};

use super::{manager::Manager, migration::MigrationTrait};

#[async_trait::async_trait]
pub trait MigratorTrait {
    fn migrators(&self) -> Vec<Box<dyn MigrationTrait>>;

    async fn register<D: DbManager + Send + 'static>(
        &self, mut db_manage: D,
    ) -> Result<D, mongodb::error::Error> {
        let manager = Manager::builder().db(db_manage.get_db()).build();
        for migrate in self.migrators() {
            let _ = migrate.migrate(&manager).await?;
        }
        let Manager { db, collections } = manager;
        db_manage.extent_collections(db, collections);
        Ok(db_manage)
    }
}

pub trait DbManager {
    fn get_db(&mut self) -> Database;
    fn extent_collections<I: IntoIterator<Item = (TypeId, Collection<()>)>>(
        &mut self, db: Database, iter: I,
    );
}
