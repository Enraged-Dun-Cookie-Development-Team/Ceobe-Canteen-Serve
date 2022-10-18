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
