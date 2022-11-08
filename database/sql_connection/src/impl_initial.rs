use std::future::Future;

use database_traits::{
    initial::{
        DatabaseInitial, DatabaseInitialBasic, DatabaseInitialConnect,
        DatabaseInitialMigration,
    },
    BoxedResultFuture,
};

use crate::{
    config,
    static_vars::{connect_to_sql_database, get_sql_database},
    DbOptionsConfig,
};

pub struct SqlDatabase;
pub struct SqlDatabaseBuilder;

impl DatabaseInitialBasic for SqlDatabase {
    type Builder = SqlDatabaseBuilder;
    type Error = sea_orm::DbErr;
}

impl<C> DatabaseInitialConnect<C> for SqlDatabase
where
    C: config::DbConnectConfig + DbOptionsConfig + 'static,
{
    type ConnectFuture<'p> =
        BoxedResultFuture<'p, Self::Builder, Self::Error>;

    fn start_connect(params: &C) -> Self::ConnectFuture<'_> {
        Box::pin(async {
            connect_to_sql_database(params).await?;
            Ok(SqlDatabaseBuilder)
        })
    }
}

impl<'p, M, Fut> DatabaseInitialMigration<'p, M> for SqlDatabase
where
    M: 'p,
    M: FnOnce(&'static sea_orm::DatabaseConnection) -> Fut,
    Fut: Future<Output = Result<(), Self::Error>> + 'p,
{
    type MigrateFuture = BoxedResultFuture<'p, Self::Builder, Self::Error>;

    fn apply_migration(
        builder: Self::Builder, params: M,
    ) -> Self::MigrateFuture {
        Box::pin(async {
            let db = get_sql_database();
            params(db).await?;
            Ok(builder)
        })
    }
}

impl DatabaseInitial for SqlDatabase {
    type BuildResult = ();

    fn build(_: Self::Builder) -> Result<Self::BuildResult, Self::Error> {
        Ok(())
    }
}
