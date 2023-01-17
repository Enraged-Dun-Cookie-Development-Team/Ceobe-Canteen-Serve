use std::net::IpAddr;

pub trait FetcherLogicConfig {
    fn schedule_host(&self) -> IpAddr;

    fn schedule_port(&self) -> u16;
}
