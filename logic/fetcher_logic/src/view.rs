use serde::{Deserialize, Serialize};
use serde_json::Value;
use sql_models::fetcher::datasource_config::models::model_datasource_config::{DataSourceForFetcherConfig, BackendDatasource};
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
    pub interval: Option<u64>,
    #[builder(default = None)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_by_time_range: Option<Vec<TimeRange>>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, TypedBuilder, PartialEq, Eq,
)]
pub struct TimeRange {
    pub time_range: Vec<String>,
    pub interval: u64,
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

// 映射单个id
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct OneIdReq {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PlatformAndDatasourceArrayResp {
    pub platform_list: Vec<String>,
    pub datasource_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DatasourceListFilterCondReq {
    pub platform: Option<String>,
    pub datasource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DatasourceList {
    pub id: i32,
    pub platform: String,
    pub datasource: String,
    pub nickname: String,
    pub avatar: String,
    pub config: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DatasourceWithNameResp {
    pub id: i32,
    pub nickname: String,
    pub config: Value,
}

impl From<BackendDatasource> for DatasourceList {
    fn from(
        BackendDatasource {
            id,
            platform,
            datasource,
            nickname,
            avatar,
            config,
        }: BackendDatasource,
    ) -> Self {
        Self {
            id,
            platform,
            datasource,
            nickname,
            avatar,
            config: serde_json::from_str(&config).unwrap(),
        }
    }
}

impl From<DataSourceForFetcherConfig> for DatasourceWithNameResp {
    fn from(
        DataSourceForFetcherConfig {
            id,
            nickname,
            config,
        }: DataSourceForFetcherConfig,
    ) -> Self {
        Self {
            id,
            nickname,
            config: serde_json::from_str(&config).unwrap(),
        }
    }
}
