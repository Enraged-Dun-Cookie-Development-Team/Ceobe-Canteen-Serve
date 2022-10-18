use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// 用于构造http监听的配置文件信息
pub trait HttpConfig {
    fn host(&self) -> IpAddr;
    fn port(&self) -> u16;
    fn socket(&self) -> SocketAddr {
        SocketAddr::new(HttpConfig::host(self), HttpConfig::port(self))
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct HttpListenConfig {
    #[serde(default = "host_default")]
    pub(crate) host: IpAddr,
    #[serde(default = "port_default")]
    pub(crate) port: u16,
}

impl Default for HttpListenConfig {
    fn default() -> Self {
        Self {
            host: host_default(),
            port: port_default(),
        }
    }
}

impl HttpConfig for HttpListenConfig {
    fn host(&self) -> IpAddr { self.host }

    fn port(&self) -> u16 { self.port }
}

fn host_default() -> IpAddr { IpAddr::V4(Ipv4Addr::LOCALHOST) }
fn port_default() -> u16 { 8000 }
