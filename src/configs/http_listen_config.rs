/// 用于构造http监听的配置文件信息
pub trait HttpConfig: serde::de::DeserializeOwned {
    fn host(&self) -> &str;
    fn port(&self) -> u16;
    fn url(&self) -> String;
}

#[derive(Debug, serde::Deserialize)]
pub struct HttpListenConfig {
    #[serde(default = "host_default")]
    pub(crate) host: String,
    #[serde(default = "port_default")]
    pub(crate) port: u16,
}

impl HttpConfig for HttpListenConfig {
    fn host(&self) -> &str { &self.host }
    fn port(&self) -> u16 { self.port }

    fn url(&self) -> String {
        let url = format!(
            "{host}:{port}",
            host = &self.host,
            port = self.port
        );
        url
    }
}

fn host_default() -> String { "127.0.0.1".into() }
fn port_default() -> u16 { 8000 }
