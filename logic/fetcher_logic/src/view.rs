use serde::{Serialize, Deserialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, PartialEq)]
pub struct Group {
    pub name: String,
    #[serde(rename="type", alias="type")]
    pub ty: String,
    pub datasource: Vec<i32>,
    #[builder(default = None)]
    pub interval: Option<i32>,
    #[builder(default = Value::Null)]
    pub interval_by_time_range: Value
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MaxLiveNumberResp {
    pub fetcher_live_number: i8
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Server {
    pub groups: Vec<Group>
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BackFetcherConfig {
    pub number: i8,
    pub server: Vec<Server>
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PlatformFilterReq {
    pub type_id: String
}