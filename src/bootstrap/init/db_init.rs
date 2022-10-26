use axum_starter::{prepare, PreparedEffect};
use database_traits::initial::{connect_db, connect_db_with_migrate};
use mongo_migration::mongo_connection::{self, MongoDbConfig, MongoDbError};
use orm_migrate::{
    sql_connection::{sea_orm::DbErr, DbConfig, SqlDatabase},
    Migrator, MigratorTrait,
};
use redis_connection::{RedisDatabase, RedisDbConfig, RedisError};

use crate::{
    bootstrap::default_user::create_default_user,
    configs::first_user::FirstUserConfig,
};

/// 连接mysql数据库并且做一次migrate up
#[prepare(box MysqlDbConnect 'arg)]
async fn connect_mysql_db_with_migrate<'arg>(
    database: &'arg DbConfig, admin_user: &'arg FirstUserConfig,
) -> Result<impl PreparedEffect, DbErr> {
    connect_db_with_migrate::<SqlDatabase, _, _>(database, |db| {
        async {
            Migrator::up(db, None).await?;
            log::info!("完成对Mysql数据库进行migration操作");
            // 创建初始后台用户
            create_default_user(db, admin_user).await;
            Ok(())
        }
    })
    .await?;
    Ok(())
}

/// 连接mongodb数据库
#[prepare(box MongoDbConnect 'arg)]
async fn connect_mongo_db<'arg>(
    mongodb: &'arg MongoDbConfig,
) -> Result<impl PreparedEffect, MongoDbError> {
    connect_db_with_migrate::<mongo_connection::DatabaseManage, _, _>(
        mongodb,
        mongo_migration::Migrator,
    )
    .await?;
    Ok(())
}

/// 连接Redis数据库
#[allow(unused)]
#[prepare(box RedisDbConnect 'arg)]
async fn connect_redis_db<'arg>(
    database: &'arg RedisDbConfig,
) -> Result<impl PreparedEffect, RedisError> {
    connect_db::<RedisDatabase, _>(database).await?;
    Ok(())
}
