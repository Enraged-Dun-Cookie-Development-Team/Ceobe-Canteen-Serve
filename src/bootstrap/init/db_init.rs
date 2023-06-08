use axum_starter::prepare;
use database_traits::initial::{connect_db, connect_db_with_migrate};
use mongo_migration::mongo_connection::{self, MongoDbConfig, MongoDbError};
use orm_migrate::{
    sql_connection::{sea_orm::DbErr, DbConfig, SqlDatabase},
    Migrator, MigratorTrait,
};
use redis_connection::{RedisDatabase, RedisDbConfig, RedisError};
use tracing::instrument;

use crate::{
    bootstrap::default_user::create_default_user,
    configs::first_user::FirstUserConfig,
};

/// 连接mysql数据库并且做一次migrate up
#[prepare(box MysqlDbConnect? 'arg)]
#[instrument(skip_all, name = "connect-and-migrate-mysql")]
async fn connect_mysql_db_with_migrate<'arg>(
    database: &'arg DbConfig, admin_user: &'arg FirstUserConfig,
) -> Result<(), DbErr> {
    connect_db_with_migrate::<SqlDatabase, _, _>(database, |db| {
        async move {
            Migrator::up(db, None).await?;
            // 创建初始后台用户
            create_default_user(db, admin_user).await;
            Ok(())
        }
    })
    .await?;
    Ok(())
}

/// 连接mongodb数据库
#[prepare(box MongoDbConnect? 'arg)]
async fn connect_mongo_db<'arg>(
    mongodb: &'arg MongoDbConfig,
) -> Result<(), MongoDbError> {
    connect_db_with_migrate::<mongo_connection::DatabaseManage, _, _>(
        mongodb,
        mongo_migration::Migrator,
    )
    .await?;
    Ok(())
}

/// 连接Redis数据库
#[prepare(box RedisDbConnect? 'arg)]
async fn connect_redis_db<'arg>(
    database: &'arg RedisDbConfig,
) -> Result<(), RedisError> {
    connect_db::<RedisDatabase, _>(database).await?;
    Ok(())
}
