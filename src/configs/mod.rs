pub mod auth_config;
pub mod first_user;
pub mod http_listen_config;
pub mod logger;
pub mod resp_result_config;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr, SocketAddrV6, IpAddr};

use axum_starter::{Provider, ServeAddress, ServerEffect};
use mongo_migration::mongo_connection::MongoDbConfig;
use orm_migrate::sql_connection::DbConfig;
use serde::Deserialize;

use self::{
    auth_config::AuthConfig, first_user::FirstUserConfig,
    http_listen_config::HttpListenConfig, logger::LoggerConfig,
    resp_result_config::RespResultConfig,
};

pub const CONFIG_FILE_TOML: &str = "./Config.toml";
pub const CONFIG_FILE_JSON: &str = "./Config.json";
pub const CONFIG_FILE_YAML: &str = "./Config.yaml";

#[derive(Debug, Deserialize, Provider)]

pub struct GlobalConfig {
    /// 数据库连接相关配置
    #[serde(alias = "db")]
    #[provider(transparent, ref)]
    pub database: DbConfig,
    #[serde(alias = "mongo")]
    #[provider(transparent, ref)]
    pub mongodb: MongoDbConfig,
    /// 日志文件相关配置
    #[serde(alias = "log")]
    #[provider(transparent, ref)]
    pub logger: LoggerConfig,
    /// resp Result
    #[serde(alias = "rresult")]
    #[provider(transparent, ref)]
    pub resp_result: RespResultConfig,
    #[serde(alias = "auth", default = "Default::default")]
    #[provider(transparent, ref)]
    pub user_auth: AuthConfig,
    #[serde(alias = "user")]
    #[provider(transparent, ref)]
    pub admin_user: FirstUserConfig,
    #[serde(alias = "http", default = "Default::default")]
    #[provider(transparent, ref)]
    pub http_listen: HttpListenConfig,
}


impl<'r> Provider<'r, SocketAddr> for GlobalConfig {
    fn provide(&'r self) -> SocketAddr {
        match self.http_listen.host {
            IpAddr::V4(ip4) => SocketAddr::V4(SocketAddrV4::new(ip4, self.http_listen.port)),
            IpAddr::V6(ip6) => SocketAddr::V6(SocketAddrV6::new(ip6, self.http_listen.port,0,0 ))
        }
    }
}

impl ServeAddress for GlobalConfig {
    type Address = SocketAddr;

    fn get_address(&self) -> Self::Address {
        match self.http_listen.host {
            IpAddr::V4(ip4) => SocketAddr::V4(SocketAddrV4::new(ip4, self.http_listen.port)),
            IpAddr::V6(ip6) => SocketAddr::V6(SocketAddrV6::new(ip6, self.http_listen.port,0,0 ))
        }
    }
}

impl ServerEffect for GlobalConfig {}