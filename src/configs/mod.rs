pub mod actix_logger;
pub mod auth_config;
pub mod first_user;
pub mod http_listen_config;
pub mod logger;
pub mod resp_result_config;
use mongo_migration::mongo_models::mongo_connection::MongoDbConfig;
use serde::Deserialize;
use sql_connection::DbConfig;

use self::{
    auth_config::AuthConfig, first_user::FirstUserConfig,
    http_listen_config::HttpListenConfig, logger::LoggerConfig,
    resp_result_config::RespResultConfig,
};

pub const CONFIG_FILE_TOML: &str = "./Config.toml";
pub const CONFIG_FILE_JSON: &str = "./Config.json";
pub const CONFIG_FILE_YAML: &str = "./Config.yaml";

#[derive(Debug, Deserialize)]

pub struct GlobalConfig {
    /// 数据库连接相关配置
    #[serde(alias = "db")]
    pub database: DbConfig,
    #[serde(alias = "mongo")]
    pub mongodb: MongoDbConfig,
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
    pub http_listen: HttpListenConfig,
}
