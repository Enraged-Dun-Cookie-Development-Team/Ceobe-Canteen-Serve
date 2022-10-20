use axum_starter::{prepare, PreparedEffect};
use database_traits::initial::connect_db_with_migrate;
use mongo_migration::mongo_connection::{MongoDbConfig, self, MongoDbError};
use orm_migrate::{sql_connection::{DbConfig, sea_orm::DbErr, SqlDatabase}, Migrator, MigratorTrait};

use crate::{configs::first_user::FirstUserConfig, bootstrap::default_user::create_default_user};

/// 连接mysql数据库并且做一次migrate up
#[prepare(MysqlDbConnect 'arg)]
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
#[prepare(MongoDbConnect 'arg)]
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
