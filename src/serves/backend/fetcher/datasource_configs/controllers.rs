use axum::{Json, extract::Query};
use checker::CheckExtract;
use futures::future;
use orm_migrate::{sql_models::fetcher::{platform_config::operate::FetcherPlatformConfigSqlOperate, datasource_config::{checkers::datasource_config_data::FetcherDatasourceConfig, operate::FetcherDatasourceConfigSqlOperate, models::model_datasource_config::{Model, BackendDatasource}}}, sql_connection::SqlConnect};
use page_size::response::{ListWithPageInfo, GenerateListWithPageInfo};
use resp_result::{resp_try, rtry};
use resp_result::MapReject;
use tracing::instrument;
use super::{error::{DatasourceConfigRResult, DatasourceConfigError}, PageSizePretreatment, view::{DatasourceListFilterCond, DatasourceId, DatasourcePlatformFilter}, FetcherDatasourceCheck};

use crate::{router::FetcherConfigControllers, serves::backend::fetcher::datasource_configs::view::{PlatformAndDatasourceArray, DatasourceList, DatasourceWithNameResp}};

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
    ) -> DatasourceConfigRResult<ListWithPageInfo<DatasourceList>> {
        resp_try(async {
            // 获取数据源列表
            let datasource_list = FetcherDatasourceConfigSqlOperate::find_datasource_list_by_page_size(&db, page_size, filter_cond.platform.clone(), filter_cond.datasource.clone());

            // 获取数据源数量
            let count = FetcherDatasourceConfigSqlOperate::get_datasource_total_number(&db, filter_cond.platform, filter_cond.datasource);
            // 异步获取
            let (datasource_list, count) = future::join(datasource_list, count).await;

            let datasource_list = datasource_list?.into_iter().map(Into::into).collect::<Vec<DatasourceList>>();
            let resp = datasource_list.with_page_info(page_size, count?);

            Ok(resp)
        })
        .await
    }

    // 上传数据源配置
    #[instrument(ret, skip(db))]
    pub async fn create_datasource_config(
        db: SqlConnect,
        CheckExtract(datasource_config): FetcherDatasourceCheck
    ) -> DatasourceConfigRResult<()> {
        rtry!(fetcher_logic::implement::create_datasource_config(&db, datasource_config).await);
        Ok(()).into()
    }

    // 更新数据源配置
    #[instrument(ret, skip(db))]
    pub async fn update_datasource_config(
        db: SqlConnect,
        CheckExtract(datasource_config): FetcherDatasourceCheck
    ) -> DatasourceConfigRResult<()> {
        rtry!(FetcherDatasourceConfigSqlOperate::update_platform_config(&db, datasource_config).await);
        Ok(()).into()
    }

     // 删除数据源配置
     #[instrument(ret, skip(db))]
     pub async fn delete_datasource_config(
         db: SqlConnect,
         MapReject(datasource): MapReject<Json<DatasourceId>, DatasourceConfigError>
     ) -> DatasourceConfigRResult<()> {
         rtry!(fetcher_logic::implement::delete_datasource_by_id(&db, datasource.id).await);
         Ok(()).into()
     }

    // 获取数据源配置全列表(包含id、名字、config)
    #[instrument(ret, skip(db))]
    pub async fn get_datasource_name_list(
        db: SqlConnect,
        MapReject(filter): MapReject<Query<DatasourcePlatformFilter>, DatasourceConfigError>
    ) -> DatasourceConfigRResult<Vec<DatasourceWithNameResp>> {
        resp_try(async {
            let list = FetcherDatasourceConfigSqlOperate::find_datasource_list_by_platform(&db, filter.type_id).await?;
            let resp = list.into_iter().map(Into::into).collect();
            Ok(resp)
        })
        .await
    }
}