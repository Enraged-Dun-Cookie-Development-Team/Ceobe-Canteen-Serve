use std::net::IpAddr;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScheduleNotifierConfig {
    #[serde(alias = "host")]
    schedule_host: IpAddr,

    #[serde(alias = "port", default = "default_port")]
    schedule_port: u16,
}

fn default_port() -> u16 { 80 }

impl fetcher_logic::config::FetcherLogicConfig for ScheduleNotifierConfig {
    fn schedule_host(&self) -> std::net::IpAddr { self.schedule_host }

    fn schedule_port(&self) -> u16 { self.schedule_port }
}
