use std::future::Future;

use database_traits::initial::{DatabaseInitialBasic, DatabaseInitialConnect, DatabaseInitial};
use redis::RedisError;

use crate::{static_var::connect_to_redis_database, config};


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
        impl Future<Output = Result<Self::Builder, Self::Error>> + 'p;

    fn start_connect(params: &C) -> Self::ConnectFuture<'_> {
        async {
            connect_to_redis_database(params).await?;
            Ok(RedisDatabaseBuilder)
        }
    }
}


impl DatabaseInitial for RedisDatabase {
    type BuildResult = ();

    fn build(_: Self::Builder) -> Result<Self::BuildResult, Self::Error> {
        Ok(())
    }
}
