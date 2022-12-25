use futures::future;
use orm_migrate::{sql_models::fetcher::{platform_config::operate::FetcherPlatformConfigSqlOperate, datasource_config::{checkers::datasource_config_data::FetcherDatasourceConfig, operate::FetcherDatasourceConfigSqlOperate}}, sql_connection::SqlConnect};
use resp_result::resp_try;
use tracing::instrument;
use super::error::DatasourceConfigRResult;

use crate::{router::FetcherConfigControllers, serves::backend::fetcher::datasource_configs::view::PlatformAndDatasourceArray};

impl FetcherConfigControllers {
    #[instrument(ret, skip(db))]
    pub async fn get_platform_and_datasource_list(
        db: SqlConnect
    ) -> DatasourceConfigRResult<PlatformAndDatasourceArray> {
        resp_try(async {
            // 获取平台列表
            let platform_list = FetcherPlatformConfigSqlOperate::find_platform_list(&db);

            // 获取数据源数量
            let datasource_list = FetcherDatasourceConfigSqlOperate::find_datasource_type_list(&db);
            // 异步获取
            let (platform_list, datasource_list) = future::join(platform_list, datasource_list).await;

            let resp = PlatformAndDatasourceArray {
                platform_list: datasource_list?,
                datasource_list: platform_list?
            };

            Ok(resp)
        })
        .await
    }
}