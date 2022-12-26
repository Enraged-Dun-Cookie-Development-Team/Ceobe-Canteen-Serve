use axum::{Json, extract::Query};
use checker::CheckExtract;
use futures::future;
use orm_migrate::{sql_models::fetcher::{platform_config::operate::FetcherPlatformConfigSqlOperate, datasource_config::{checkers::datasource_config_data::FetcherDatasourceConfig, operate::FetcherDatasourceConfigSqlOperate, models::model_datasource_config::{Model, BackendDatasource}}}, sql_connection::SqlConnect};
use page_size::response::{ListWithPageInfo, GenerateListWithPageInfo};
use resp_result::resp_try;
use resp_result::MapReject;
use tracing::instrument;
use super::{error::{DatasourceConfigRResult, DatasourceConfigError}, PageSizePretreatment, view::DatasourceListFilterCond};

use crate::{router::FetcherConfigControllers, serves::backend::fetcher::datasource_configs::view::PlatformAndDatasourceArray};

impl FetcherConfigControllers {
    // 获取平台与数据源类型列表
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

    // 获取数据源列表
    #[instrument(ret, skip(db))]
    pub async fn get_datasource_list(
        db: SqlConnect, CheckExtract(page_size): PageSizePretreatment,
        MapReject(filter_cond): MapReject<
            Query<DatasourceListFilterCond>,
            DatasourceConfigError,
        >,
    ) -> DatasourceConfigRResult<ListWithPageInfo<BackendDatasource>> {
        resp_try(async {
            // 获取数据源列表
            let datasource_list = FetcherDatasourceConfigSqlOperate::find_datasource_list_by_page_size(&db, page_size, filter_cond.platform.clone(), filter_cond.datasource.clone());

            // 获取数据源数量
            let count = FetcherDatasourceConfigSqlOperate::get_datasource_total_number(&db, filter_cond.platform, filter_cond.datasource);
            // 异步获取
            let (datasource_list, count) = future::join(datasource_list, count).await;

            let resp = datasource_list?.with_page_info(page_size, count?);

            Ok(resp)
        })
        .await
    }
}