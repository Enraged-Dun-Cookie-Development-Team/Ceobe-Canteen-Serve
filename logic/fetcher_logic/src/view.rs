use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use sql_models::fetcher::{
    config::models,
    datasource_config::models::model_datasource_config::{
        BackendDatasource, DataSourceForFetcherConfig,
    },
};
use typed_builder::TypedBuilder;

/// 返回蹲饼器配置与上传蹲饼器配置的组数据
#[derive(
    Debug, Clone, Serialize, Deserialize, TypedBuilder, PartialEq, Eq,
)]
pub struct Group {
    pub name: String,
    pub platform: String,
    #[builder(default)]
    #[serde(rename = "datasource")]
    pub data_source: Vec<i32>,
    #[builder(default = None)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<u64>,
    #[builder(default = None)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_by_time_range: Option<Vec<TimeRange>>,
}

impl TryFrom<models::model_config::Model> for Group {
    type Error = serde_json::Error;

    fn try_from(
        models::model_config::Model {
            group_name,
            platform,
            interval,
            interval_by_time_range,
            ..
        }: models::model_config::Model,
    ) -> Result<Self, Self::Error> {
        Ok(Group {
            name: group_name,
            platform,
            data_source: vec![],
            interval,
            interval_by_time_range: serde_json::from_str(
                &interval_by_time_range,
            )?,
        })
    }
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

impl<GI: IntoIterator<Item = Group>> From<GI> for Server {
    fn from(g: GI) -> Self {
        Self::builder().groups(g.into_iter().collect()).build()
    }
}

/// 单种存活数量下多个蹲饼器配置
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BackEndFetcherConfig {
    pub number: i8,
    pub server: Vec<Server>,
}

impl BackEndFetcherConfig {
    pub fn new<S: IntoIterator<Item = Server>>(
        number: i8, server: S,
    ) -> Self {
        Self {
            number,
            server: server.into_iter().collect(),
        }
    }
}

/// 蹲饼器配置根结构体
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PlatformFilterReq {
    pub type_id: String,
}

/// 映射单个id
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
    #[serde(deserialize_with = "empty_change_to_none")]
    pub platform: Option<String>,
    #[serde(deserialize_with = "empty_change_to_none")]
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
    pub datasource: String,
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
            datasource,
        }: DataSourceForFetcherConfig,
    ) -> Self {
        Self {
            id,
            nickname,
            config: serde_json::from_str(&config).unwrap(),
            datasource,
        }
    }
}

fn empty_change_to_none<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(d)?;
    Ok(match value.as_deref() {
        Some("") | None => None,
        _ => value,
    })
}
