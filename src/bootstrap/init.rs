use axum::handler::Handler;
use axum_starter::{
    graceful::SetGraceful,
    prepare,
    router::{Fallback, Nest},
    PreparedEffect,
};
use database_traits::initial::connect_db_with_migrate;
use futures::FutureExt;
use mongo_migration::mongo_connection::{self, MongoDbConfig, MongoDbError};
use orm_migrate::{
    sql_connection::{sea_orm::DbErr, DbConfig, SqlDatabase},
    Migrator, MigratorTrait,
};
use tokio::sync::oneshot;

use crate::{
    bootstrap::default_user::create_default_user,
    configs::{
        auth_config::AuthConfig, first_user::FirstUserConfig,
        logger::LoggerConfig, resp_result_config::RespResultConfig,
    },
    error::not_exist,
    router,
    utils::user_authorize,
};

/// 日志配置
#[prepare(LoggerInitialization 'arg)]
fn logger_register(logger: &'arg LoggerConfig) -> impl PreparedEffect {
    logger.register_logger();
}

/// 请求rresult配置
#[prepare(RResultConfig 'arg)]
async fn resp_conf(
    resp_result: &'arg RespResultConfig,
) -> impl PreparedEffect {
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

/// 配置router
#[prepare(RouteV1)]
fn router_v1() -> impl PreparedEffect {
    Nest::new("/api/v1", router::root_route())
}

/// 配置Fallback
#[prepare(RouterFallback)]
fn router_fallback() -> impl PreparedEffect {
    Fallback::new(not_exist.into_service())
}

pub async fn graceful_shutdown() -> impl PreparedEffect {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(_) => {
                log::info!("收到退出信号");
                tx.send(())
            }
            Err(err) => {
                log::error!("等待退出信号异常 {err}");
                tx.send(())
            }
        }
    });
    tokio::task::yield_now().await;

    SetGraceful::new(rx.map(|_| ()))
}
