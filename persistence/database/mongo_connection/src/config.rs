use std::collections::HashMap;

pub trait DbConnectConfig: serde::de::DeserializeOwned {
    fn scheme(&self) -> &str;
    fn username(&self) -> &str;
    fn password(&self) -> &str;
    fn host(&self) -> &str;
    fn port(&self) -> u16;
    fn name(&self) -> &str;
    fn query(&self) -> &HashMap<String, String>;
}

#[derive(Debug, serde::Deserialize)]
pub struct MongoDbConfig {
    username: String,
    password: String,
    #[serde(default = "host_default")]
    host: String,
    #[serde(default = "port_default")]
    port: u16,
    db_name: String,
    query: HashMap<String, String>,
}

impl DbConnectConfig for MongoDbConfig {
    fn scheme(&self) -> &str { "mongodb" }

    fn username(&self) -> &str { &self.username }

    fn password(&self) -> &str { &self.password }

    fn host(&self) -> &str { &self.host }

    fn port(&self) -> u16 { self.port }

    fn name(&self) -> &str { &self.db_name }

    fn query(&self) -> &HashMap<String, String> { &self.query}
}

fn host_default() -> String { "localhost".into() }

fn port_default() -> u16 { 27017 }
