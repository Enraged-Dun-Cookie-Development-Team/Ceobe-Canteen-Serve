/// 用于构造database的配置文件信息
pub trait DbConnectConfig: serde::de::DeserializeOwned {
    fn scheme(&self) -> &str;
    fn username(&self) -> &str;
    fn password(&self) -> &str;
    fn host(&self) -> &str;
    fn port(&self) -> u16;
    fn name(&self) -> &str;
}

/// 配置数据库连接池数据
pub trait DbOptionsConfig {
    fn max_conn(&self) -> Option<u32> { None }
    fn min_conn(&self) -> Option<u32> { None }
    fn sql_logger(&self) -> bool { false }
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct DbConfig {
    pub(crate) scheme: String,
    pub(crate) username: String,
    pub(crate) password: String,
    #[serde(default = "host_default")]
    pub(crate) host: String,
    #[serde(default = "port_default")]
    pub(crate) port: u16,
    pub(crate) name: String,

    pub(crate) max_conn: Option<u32>,
    pub(crate) min_conn: Option<u32>,
    #[serde(default = "logger_default")]
    pub(crate) logger: bool,
}

impl DbConnectConfig for DbConfig {
    fn scheme(&self) -> &str { &self.scheme }

    fn username(&self) -> &str { &self.username }

    fn password(&self) -> &str { &self.password }

    fn host(&self) -> &str { &self.host }

    fn port(&self) -> u16 { self.port }

    fn name(&self) -> &str { &self.name }
}

impl DbOptionsConfig for DbConfig {
    fn max_conn(&self) -> Option<u32> { self.max_conn }

    fn min_conn(&self) -> Option<u32> { self.min_conn }

    fn sql_logger(&self) -> bool { self.logger }
}

fn logger_default() -> bool { false }

fn host_default() -> String { "localhost".into() }
fn port_default() -> u16 { 3306 }
