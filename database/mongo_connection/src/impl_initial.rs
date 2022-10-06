use database_traits::initial::{
    DatabaseInitial, DatabaseInitialBasic, DatabaseInitialConnect,
    DatabaseInitialMigration,
};
use futures::Future;

use crate::{DatabaseManage, DbConnectConfig, MongoConnectBuilder, MongoErr};

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

impl<'p, M> DatabaseInitialMigration<'p, M> for DatabaseManage
where
    M: mongo_migrate_util::MigratorTrait + Sync + Send + 'p,
{
    type MigrateFuture =
        impl Future<Output = Result<Self::Builder, Self::Error>>;

    fn apply_migration(
        builder: Self::Builder, params: M,
    ) -> Self::MigrateFuture {
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
