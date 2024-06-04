use axum_starter::{prepare, state::AddState};
use persistence::{
    connect::{connect_db, connect_db_with_migrate},
    mongodb,
    mongodb::{MongoConnect, MongoDbConfig, MongoDbError},
    mysql::{
        sea_orm::DbErr, DbConfig, Migrator, MigratorTrait, SqlConnect,
        SqlDatabase,
    },
    redis::{RedisConnect, RedisDatabase, RedisDbConfig, RedisError},
};
use tracing::instrument;

use crate::{
    bootstrap::default_user::create_default_user,
    configs::first_user::FirstUserConfig,
};

/// 连接mysql数据库并且做一次migrate up
#[instrument(skip_all, name = "connect-and-migrate-mysql")]
#[prepare(box MysqlDbConnect?)]
async fn connect_mysql_db_with_migrate(
    database: &DbConfig, admin_user: &FirstUserConfig,
) -> Result<AddState<SqlConnect>, DbErr> {
    connect_db_with_migrate::<SqlDatabase, _, _>(database, |db| {
        async move {
            Migrator::up(db, None).await?;
            // 创建初始后台用户
            create_default_user(db, admin_user).await;
            Ok(())
        }
    })
    .await?;

    Ok(AddState::new(SqlConnect))
}

/// 连接mongodb数据库
#[prepare(box MongoDbConnect?)]
async fn connect_mongo_db(
    mongodb: &MongoDbConfig,
) -> Result<AddState<MongoConnect>, MongoDbError> {
    connect_db_with_migrate::<mongodb::DatabaseManage, _, _>(
        mongodb,
        mongodb::Migrator,
    )
    .await?;
    Ok(AddState::new(MongoConnect))
}

/// 连接Redis数据库
#[prepare(box RedisDbConnect?)]
async fn connect_redis_db(
    database: &RedisDbConfig,
) -> Result<AddState<RedisConnect>, RedisError> {
    connect_db::<RedisDatabase, _>(database).await?;
    Ok(AddState::new(RedisConnect::from_static()))
}
