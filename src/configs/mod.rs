pub mod actix_logger;
pub mod logger;
pub mod resp_result_config;
use serde::Deserialize;

use self::{logger::LoggerConfig, resp_result_config::RespResultConfig};
use crate::{
    database::config::DbConfig, utils::mongodb_utils::config::MongoDbConfig,
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
}
