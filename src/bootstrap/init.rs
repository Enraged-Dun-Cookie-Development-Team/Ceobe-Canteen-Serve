use axum_starter::{prepare, PreparedEffect};
use database_traits::initial::connect_db_with_migrate;
use mongo_migration::mongo_connection::{self, MongoDbConfig, MongoDbError};
use orm_migrate::{
    sql_connection::{sea_orm::DbErr, DbConfig, SqlDatabase},
    Migrator, MigratorTrait,
};

use crate::{
    bootstrap::create::create_default_user,
    configs::{
        auth_config::AuthConfig, first_user::FirstUserConfig,
        logger::LoggerConfig, resp_result_config::RespResultConfig,
    },
    utils::user_authorize,
};

/// 日志配置
#[prepare(LoggerRegister 'arg)]
async fn logger_register(logger: &'arg LoggerConfig) -> impl PreparedEffect {
    logger.register_logger();
}

/// 请求返回resp配置
#[prepare(RespConfig 'arg)]
async fn resp_conf(resp_result: &'arg RespResultConfig) -> impl PreparedEffect {
    resp_result::set_config(resp_result);
}

/// 鉴权配置
#[prepare(BackAuthConfig 'arg)]
async fn backend_user_auth_conf(
    user_auth: &'arg AuthConfig,
) -> impl PreparedEffect {
    user_authorize::set_auth_config(user_auth);
}

/// 连接mysql数据库并且做一次migrate up
#[prepare(SqlDatabaseConnect 'arg)]
async fn connect_sql_db_with_migrate<'arg>(
    database: &'arg DbConfig, admin_user: &'arg FirstUserConfig,
) -> Result<impl PreparedEffect, DbErr> {
    connect_db_with_migrate::<SqlDatabase, _, _>(database, |db| {
        async {
            Migrator::up(db, None).await?;
            log::info!("完成对Mysql数据库进行migration操作");
            create_default_user(db, admin_user).await;
            Ok(())
        }
    })
    .await?;
    Ok(())
}

/// 连接mongodb数据库
#[prepare(MongoDatabaseConnect 'arg)]
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
