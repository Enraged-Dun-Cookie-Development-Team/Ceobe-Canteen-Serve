use async_trait::async_trait;
use futures::Future;

use super::ModelRegister;
use crate::utils::mongodb_utils::db_manager::DbBuild;

pub struct MongoRegister<F, Fut>(F)
where
    F: FnOnce(DbBuild) -> Fut + Send,
    Fut: Future<Output = DbBuild> + Send;

#[async_trait]
impl<F, Fut> ModelRegister for MongoRegister<F, Fut>
where
    F: FnOnce(DbBuild) -> Fut + Send,
    Fut: Future<Output = DbBuild> + Send,
{
    async fn register_mongo(self, db: DbBuild) -> DbBuild { self.0(db).await }
}

pub fn as_mongo_register<F, Fut>(fun: F) -> MongoRegister<F, Fut>
where
    F: FnOnce(DbBuild) -> Fut + Send,
    Fut: Future<Output = DbBuild> + Send,
{
    MongoRegister(fun)
}
