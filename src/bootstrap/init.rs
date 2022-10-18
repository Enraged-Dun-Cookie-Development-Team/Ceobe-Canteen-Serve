use axum_starter::{prepare, PreparedEffect};
use database_traits::initial::connect_db_with_migrate;
use orm_migrate::{
    sql_connection::{DbConfig, SqlDatabase, sea_orm::DbErr},
    Migrator, MigratorTrait,
};

use crate::{
    bootstrap::create::create_default_user,
    configs::{first_user::FirstUserConfig, resp_result_config::RespResultConfig, auth_config::AuthConfig, logger::LoggerConfig}, utils::user_authorize,
};

/// 日志配置
#[prepare(LoggerRegister 'arg)]
fn logger_register(logger: &'arg LoggerConfig) -> impl PreparedEffect {
    logger.register_logger();
}

/// 请求返回resp配置
#[prepare(RespConfig 'arg)]
fn resp_conf(resp_result: &'arg RespResultConfig) -> impl PreparedEffect {
    resp_result::set_config(resp_result);
}

/// 鉴权配置
#[prepare(BackAuthConfig 'arg)]
fn backend_user_auth_conf(user_auth: &'arg AuthConfig) -> impl PreparedEffect {
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
