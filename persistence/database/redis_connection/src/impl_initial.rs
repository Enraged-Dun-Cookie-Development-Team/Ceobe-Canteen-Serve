use database_traits::{
    initial::{
        DatabaseInitial, DatabaseInitialBasic, DatabaseInitialConnect,
    },
    BoxedResultFuture,
};
use redis::RedisError;

use crate::{config, static_var::connect_to_redis_database};

pub struct RedisDatabase;
pub struct RedisDatabaseBuilder;

impl DatabaseInitialBasic for RedisDatabase {
    type Builder = RedisDatabaseBuilder;
    type Error = RedisError;
}

impl<C> DatabaseInitialConnect<C> for RedisDatabase
where
    C: config::DbConnectConfig + 'static,
{
    type ConnectFuture<'p> =
        BoxedResultFuture<'p, Self::Builder, Self::Error>;

    fn start_connect(params: &C) -> Self::ConnectFuture<'_> {
        Box::pin(async {
            connect_to_redis_database(params).await?;
            Ok(RedisDatabaseBuilder)
        })
    }
}

impl DatabaseInitial for RedisDatabase {
    type BuildResult = ();

    fn build(_: Self::Builder) -> Result<Self::BuildResult, Self::Error> {
        Ok(())
    }
}
