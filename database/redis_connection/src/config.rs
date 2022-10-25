pub trait DbConnectConfig: serde::de::DeserializeOwned {
    fn scheme(&self) -> &str;
    fn username(&self) -> &str;
    fn password(&self) -> &str;
    fn host(&self) -> &str;
    fn port(&self) -> u16;
    fn db(&self) -> u8;
}

#[derive(Debug, serde::Deserialize)]
pub struct RedisDbConfig {
    username: String,
    password: String,
    #[serde(default = "host_default")]
    host: String,
    #[serde(default = "port_default")]
    port: u16,
    #[serde(default = "db_default")]
    db: u8,
}

impl DbConnectConfig for RedisDbConfig {
    fn scheme(&self) -> &str { "redis" }

    fn username(&self) -> &str { &self.username }

    fn password(&self) -> &str { &self.password }

    fn host(&self) -> &str { &self.host }

    fn port(&self) -> u16 { self.port }

    fn db(&self) -> u8 { self.db }
}

fn host_default() -> String { "localhost".into() }

fn port_default() -> u16 { 6379 }

fn db_default() -> u8 { 0 }
