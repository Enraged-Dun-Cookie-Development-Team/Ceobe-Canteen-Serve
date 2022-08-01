use std::any::TypeId;

use async_trait::async_trait;
use mongodb::{Collection, Database};

use super::manager::Manager;

#[async_trait]
pub trait MigratorTrait {
    async fn migrating(
        &self, manage: &mut Manager<'_>,
    ) -> Result<(), mongodb::error::Error>;

    async fn register<D: DbManager + Send + Sync + 'static>(
        &self, mut db_manage: D,
    ) -> Result<D, mongodb::error::Error> {
        let mut manager = Manager::new(db_manage.get_db()).await?;

        log::info!("开始执行 Migrate MongoDb");
        self.migrating(&mut manager).await?;

        log::info!("执行 Migrate MongoDb 完成");
        let collects = manager.done().await?;

        db_manage.extent_collections(collects);
        Ok(db_manage)
    }
}

pub trait DbManager {
    fn get_db(&self) -> &Database;
    fn extent_collections<I: IntoIterator<Item = (TypeId, Collection<()>)>>(
        &mut self, iter: I,
    );
}
