use std::net::{IpAddr, SocketAddr, Ipv4Addr};

/// 用于构造http监听的配置文件信息
pub trait HttpConfig {
    fn host(&self) -> IpAddr;
    fn port(&self) -> u16;
    fn url(&self) -> SocketAddr;
}

#[derive(Debug, serde::Deserialize)]
pub struct HttpListenConfig {
    #[serde(default = "host_default")]
    pub(crate) host: IpAddr,
    #[serde(default = "port_default")]
    pub(crate) port: u16,
}

impl HttpConfig for HttpListenConfig {
    fn host(&self) -> IpAddr { self.host }

    fn port(&self) -> u16 { self.port }

    fn url(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

fn host_default() -> IpAddr { IpAddr::V4(Ipv4Addr::LOCALHOST) }
fn port_default() -> u16 { 8000 }
