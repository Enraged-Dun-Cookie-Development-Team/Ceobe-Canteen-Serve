use orm_migrate::sql_models::fetcher::datasource_config::models::model_datasource_config::{BackendDatasource, DataSourceForFetcherConfig};
use serde_json::Value;

crate::quick_struct! {
    pub PlatformAndDatasourceArray {
        platform_list: Vec<String>
        datasource_list: Vec<String>
    }

    pub DatasourceListFilterCond {
        platform: Option<String>
        datasource: Option<String>
    }

    pub DatasourceList {
        id: i32
        platform: String
        datasource: String
        nickname: String
        avatar: String
        config: Value
    }

    pub DatasourceId {
        id: i32
    }

    pub DatasourcePlatformFilter {
        type_id: String
    }

    pub DatasourceWithNameResp {
        id: i32
        nickname: String
        config: Value
    }
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
