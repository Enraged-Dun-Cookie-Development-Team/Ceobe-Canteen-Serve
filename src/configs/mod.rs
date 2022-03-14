pub mod actix_logger;
pub mod logger;
use serde::Deserialize;

use crate::database::config::DbConfig;

use self::logger::LoggerConfig;

pub const CONFIG_FILE_TOML: &str = "./Config.toml";
pub const CONFIG_FILE_JSON: &str = "./Config.json";
pub const CONFIG_FILE_YAML: &str = "./Config.yaml";

#[derive(Debug, Deserialize)]

pub struct GlobalConfig {
    /// 数据库连接相关配置
    #[serde(alias = "db")]
    pub database: DbConfig,
    /// 日志文件相关配置
    #[serde(alias = "log")]
    pub logger: LoggerConfig,
}
