use std::any::TypeId;

use async_trait::async_trait;
use mongodb::{Client, Collection, Database};

use super::manager::Manager;

#[async_trait]
pub trait MigratorTrait {
    async fn migrating(
        &self, manage: &Manager<'_>,
    ) -> Result<(), mongodb::error::Error>;

    async fn register<D: DbManager + Send + Sync + 'static>(
        &self, mut db_manage: D,
    ) -> Result<D, mongodb::error::Error> {
        let mut manager =
            Manager::new(db_manage.get_client(), db_manage.get_db()).await?;

        self.migrating(&mut manager).await?;

        let collects = manager.apply_all().await?;

        db_manage.extent_collections(collects);
        Ok(db_manage)
    }
}

pub trait DbManager {
    fn get_client(&self) -> &Client;
    fn get_db(&self) -> &Database;
    fn extent_collections<I: IntoIterator<Item = (TypeId, Collection<()>)>>(
        &mut self, iter: I,
    );
}
