#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
mod config;
mod database;
mod error;
mod mongo_connect;

mod static_vars;

pub type MongoDb = mongodb::Database;
pub type MongoClient = mongodb::Client;
pub type MongoErr = mongodb::error::Error;
pub type MongoClientOptions = mongodb::options::ClientOptions;

use std::future::Future;

pub use config::{DbConnectConfig, MongoDbConfig};
pub use database::manager::{CollectionGuard, DatabaseManage};
use database_initial::{
    DatabaseInitial, DatabaseInitialBasic, DatabaseInitialConnect,
    DatabaseInitialMigration,
};
pub use error::MongoDbError;
pub use mongo_connect::MongoConnectBuilder;
pub use static_vars::{get_mongo_collection, get_mongo_database};

impl DatabaseInitialBasic for DatabaseManage {
    type Builder = MongoConnectBuilder;
    type Error = MongoErr;
}

impl<C> DatabaseInitialConnect<C> for DatabaseManage
where
    C: DbConnectConfig + 'static + Sized,
{
    type ConnectFuture<'p> =
        impl Future<Output = Result<Self::Builder, Self::Error>> + 'p;

    fn start_connect(params: &C) -> Self::ConnectFuture<'_> {
        async { MongoConnectBuilder::new(params).await }
    }
}

impl<M> DatabaseInitialMigration<M> for DatabaseManage
where
    M: mongo_migrate_util::MigratorTrait + 'static + Sync + Send,
{
    type MigrateFuture<'p> =
        impl Future<Output = Result<Self::Builder, Self::Error>> + 'p;

    fn apply_migration(
        builder: Self::Builder, params: &M,
    ) -> Self::MigrateFuture<'_> {
        async move { builder.apply_mongo_migration(params).await }
    }
}

impl DatabaseInitial for DatabaseManage {
    type BuildResult = ();

    fn build(
        builder: Self::Builder,
    ) -> Result<Self::BuildResult, Self::Error> {
        builder.build();
        Ok(())
    }
}
