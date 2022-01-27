use std::net::Ipv4Addr;

/// 用于构造databas的配置文件信息
pub trait DbConnetConfig: serde::de::DeserializeOwned {
    fn scheme(&self) -> &str;
    fn username(&self) -> &str;
    fn password(&self) -> &str;
    fn host(&self) -> Ipv4Addr;
    fn port(&self) -> u16;
    fn name(&self) -> &str;
}

pub trait DbOptionsConfig {
    fn max_conn(&self) -> u32;
    fn min_conn(&self) -> u32;
    fn sql_logger(&self) -> bool {
        false
    }
}
