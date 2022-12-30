use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// 返回蹲饼器配置与上传蹲饼器配置的组数据
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

/// 至今为止最大存活数量
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MaxLiveNumberResp {
    pub fetcher_live_number: i8,
}

/// 单个蹲饼器配置结构体
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Server {
    pub groups: Vec<Group>,
}

/// 单种存活数量下多个蹲饼器配置
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BackFetcherConfig {
    pub number: i8,
    pub server: Vec<Server>,
}

/// 蹲饼器配置根结构体
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PlatformFilterReq {
    pub type_id: String,
}
