use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct Meta {
    pub source: Source,
    pub account: Account,
    pub item: Item,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct Source {
    #[serde(rename = "type", alias = "type")]
    pub ty: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct Account {
    pub id: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct Item {
    pub id: String,
    pub url: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct Timestamp {
    pub platform: Option<i64>,
    pub platform_precision: PlatformPrecision,
    pub fetcher: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlatformPrecision {
    None,
    Day,
    Hour,
    Minute,
    Second,
    Ms,
}
