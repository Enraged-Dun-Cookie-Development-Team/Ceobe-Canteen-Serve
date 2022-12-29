use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(
    Debug, Clone, Serialize, Deserialize, TypedBuilder, PartialEq, Eq,
)]
pub struct Group {
    pub name: String,
    #[serde(rename = "type", alias = "type")]
    pub ty: String,
    pub datasource: Vec<i32>,
    #[builder(default = None)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[builder(default = Value::Null)]
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub interval_by_time_range: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MaxLiveNumberResp {
    pub fetcher_live_number: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Server {
    pub groups: Vec<Group>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BackFetcherConfig {
    pub number: i8,
    pub server: Vec<Server>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PlatformFilterReq {
    pub type_id: String,
}
