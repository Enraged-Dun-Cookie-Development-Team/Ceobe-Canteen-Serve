#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

mod config;
mod static_vars;

use std::future::Future;

pub use config::{DbConfig, DbConnectConfig, DbOptionsConfig};
use database_initial::{
    DatabaseInitial, DatabaseInitialBasic, DatabaseInitialConnect,
};
pub use sea_orm;
pub use static_vars::{
    connect_to_sql_database, get_sql_database, get_sql_transaction,
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
        impl Future<Output = Result<Self::Builder, Self::Error>> + 'p;

    fn start_connect(params: &C) -> Self::ConnectFuture<'_> {
        async {
            connect_to_sql_database(params).await?;
            Ok(SqlDatabaseBuilder)
        }
    }
}

impl DatabaseInitial for SqlDatabase {
    type BuildResult = ();

    fn build(
        _: Self::Builder,
    ) -> Result<Self::BuildResult, Self::Error> {
        Ok(())
    }
}
