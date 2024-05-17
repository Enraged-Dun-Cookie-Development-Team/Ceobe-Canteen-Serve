use std::net::SocketAddr;

use axum_starter::{Configure, Provider};
use persistence::{
    mongodb::MongoDbConfig, mysql::DbConfig, redis::RedisDbConfig,
};
use serde::Deserialize;

use self::{
    auth_config::AuthConfig,
    first_user::FirstUserConfig,
    http_listen_config::{HttpConfig, HttpListenConfig},
    logger::LoggerConfig,
    qiniu_secret::QiniuUploadConfig,
    resp_result_config::RespResultConfig,
};

pub mod auth_config;
pub mod cors_config;
pub mod first_user;
pub mod http_listen_config;
pub mod logger;
pub mod mob_config;
pub mod qiniu_secret;
pub mod qq_channel;
pub mod resp_result_config;
pub mod schedule_notifier_config;

pub const CONFIG_FILE_TOML: &str = "./Config.toml";
pub const CONFIG_FILE_JSON: &str = "./Config.json";
pub const CONFIG_FILE_YAML: &str = "./Config.yaml";

#[derive(Debug, Deserialize, Provider, Configure)]
#[conf(
    address(provide),
    logger(func = "|this|this.logger.init_log()", error = "::logger::Error"),
    server
)]
#[provider(transparent, r#ref)]
pub struct GlobalConfig {
    /// 数据库连接相关配置
    #[serde(alias = "db")]
    pub database: DbConfig,
    #[serde(alias = "mongo")]
    pub mongodb: MongoDbConfig,
    pub redis: RedisDbConfig,
    /// 日志文件相关配置
    #[serde(alias = "log")]
    pub logger: LoggerConfig,
    /// resp Result
    #[serde(alias = "rresult")]
    pub resp_result: RespResultConfig,
    #[serde(alias = "auth", default = "Default::default")]
    pub user_auth: AuthConfig,
    #[serde(alias = "user")]
    pub admin_user: FirstUserConfig,
    #[serde(alias = "http", default = "Default::default")]
    #[provider(map_to(ty = "SocketAddr", by = "HttpConfig::socket"))]
    pub http_listen: HttpListenConfig,
    #[serde(alias = "qiniu")]
    pub qiniu_secret: QiniuUploadConfig,

    #[serde(alias = "schedule")]
    pub schedule_manage: schedule_notifier_config::ScheduleNotifierConfig,
    /// mob push推送
    #[serde(alias = "mob")]
    pub mob_push: mob_config::MobPushConfig,
    #[serde(alias = "qq")]
    pub qq_channel: qq_channel::QqChannelConfig,
    #[serde(default)]
    pub cors: cors_config::CorsConfigImpl,
}
