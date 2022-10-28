use std::time::Duration;

use ahash::AHashMap;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::PlatformName;

#[derive(Debug, Serialize,TypedBuilder)]
pub struct PlatformConfigure {
    #[serde(rename = "min_request_interval")]
    #[builder(setter(transform = |interval: Duration| interval.as_millis()))]
    min_request_interval_microsecond: u128,
}

pub type Platforms = AHashMap<PlatformName, PlatformConfigure>;
