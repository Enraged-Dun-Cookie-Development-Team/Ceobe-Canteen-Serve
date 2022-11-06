use database_traits::{
    initial::{
        DatabaseInitial, DatabaseInitialBasic, DatabaseInitialConnect,
        DatabaseInitialMigration,
    },
    BoxedResultFuture,
};

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
        BoxedResultFuture<'p, Self::Builder, Self::Error>;

    fn start_connect(params: &C) -> Self::ConnectFuture<'_> {
        Box::pin(MongoConnectBuilder::new(params))
    }
}

impl<'p, M> DatabaseInitialMigration<'p, M> for DatabaseManage
where
    M: mongo_migrate_util::MigratorTrait + Sync + Send + 'p,
{
    type MigrateFuture = BoxedResultFuture<'p, Self::Builder, Self::Error>;

    fn apply_migration(
        builder: Self::Builder, params: M,
    ) -> Self::MigrateFuture {
        Box::pin(builder.apply_mongo_migration(params))
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
