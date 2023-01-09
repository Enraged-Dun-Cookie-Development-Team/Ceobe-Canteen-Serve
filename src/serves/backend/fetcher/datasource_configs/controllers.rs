use axum::{extract::Query, Json};
use checker::CheckExtract;
use fetcher_logic::{
    implements::{DatasourceConfig, FetcherConfigLogic, SuperLogic},
    view::{
        DatasourceList, DatasourceListFilterCondReq, DatasourceWithNameResp,
        OneIdReq, PlatformAndDatasourceArrayResp, PlatformFilterReq,
    },
};
use futures::future;
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::fetcher::{
        datasource_config::operate::FetcherDatasourceConfigSqlOperate,
        platform_config::operate::FetcherPlatformConfigSqlOperate,
    },
};
use page_size::response::{GenerateListWithPageInfo, ListWithPageInfo};
use resp_result::{resp_try, rtry, MapReject};
use tracing::instrument;

use super::{
    error::{DatasourceConfigError, DatasourceConfigRResult},
    FetcherDatasourceCheck, PageSizePretreatment,
};
use crate::router::FetcherConfigControllers;

impl FetcherConfigControllers {
    /// 获取平台与数据源类型列表
    #[instrument(ret, skip(db))]
    pub async fn get_platform_and_datasource_list(
        db: SqlConnect,
    ) -> DatasourceConfigRResult<PlatformAndDatasourceArrayResp> {
        resp_try(async {
            // 获取平台列表
            let platform_list =
                FetcherPlatformConfigSqlOperate::find_all(&db);

            // 获取数据源数量
            let datasource_list =
                FetcherDatasourceConfigSqlOperate::find_all_type(&db);
            // 异步获取
            let (platform_list, datasource_list) =
                future::join(platform_list, datasource_list).await;

            let resp = PlatformAndDatasourceArrayResp {
                platform_list: platform_list?,
                datasource_list: datasource_list?,
            };

            Ok(resp)
        })
        .await
    }

    /// 获取数据源列表
    #[instrument(ret, skip(db))]
    pub async fn get_datasource_list(
        db: SqlConnect, CheckExtract(page_size): PageSizePretreatment,
        MapReject(filter_cond): MapReject<
            Query<DatasourceListFilterCondReq>,
            DatasourceConfigError,
        >,
    ) -> DatasourceConfigRResult<ListWithPageInfo<DatasourceList>> {
        resp_try(async {
            // 获取数据源列表
            let datasource_list =
                FetcherDatasourceConfigSqlOperate::find_all_with_paginator(
                    &db,
                    page_size,
                    filter_cond.platform.clone(),
                    filter_cond.datasource.clone(),
                );

            // 获取数据源数量
            let count = FetcherDatasourceConfigSqlOperate::count(
                &db,
                filter_cond.platform,
                filter_cond.datasource,
            );
            // 异步获取
            let (datasource_list, count) =
                future::join(datasource_list, count).await;

            let datasource_list = datasource_list?
                .into_iter()
                .map(Into::into)
                .collect::<Vec<DatasourceList>>();
            let resp = datasource_list.with_page_info(page_size, count?);

            Ok(resp)
        })
        .await
    }

    /// 上传数据源配置
    #[instrument(ret, skip(db))]
    // #[axum::debug_handler]
    pub async fn create_datasource_config(
        db: SqlConnect,
        CheckExtract(datasource_config): FetcherDatasourceCheck,
    ) -> DatasourceConfigRResult<()> {
        resp_try(async {
            FetcherConfigLogic
                .sub_logic::<DatasourceConfig>()
                .create(&db, datasource_config)
                .await?;
            Ok(())
        })
        .await
    }

    // 更新数据源配置
    #[instrument(ret, skip(db))]
    pub async fn update_datasource_config(
        db: SqlConnect,
        CheckExtract(datasource_config): FetcherDatasourceCheck,
    ) -> DatasourceConfigRResult<()> {
        rtry!(
            FetcherDatasourceConfigSqlOperate::update(&db, datasource_config)
                .await
        );
        Ok(()).into()
    }

    // 删除数据源配置
    #[instrument(ret, skip(db))]
    pub async fn delete_datasource_config(
        db: SqlConnect,
        MapReject(datasource): MapReject<
            Json<OneIdReq>,
            DatasourceConfigError,
        >,
    ) -> DatasourceConfigRResult<()> {
        rtry!(
            FetcherConfigLogic
                .sub_logic::<DatasourceConfig>()
                .delete_by_id(&db, datasource.id)
                .await
        );
        Ok(()).into()
    }

    /// 获取数据源配置全列表(包含id、名字、config)
    #[instrument(ret, skip(db))]
    pub async fn get_datasource_name_list(
        db: SqlConnect,
        MapReject(filter): MapReject<
            Query<PlatformFilterReq>,
            DatasourceConfigError,
        >,
    ) -> DatasourceConfigRResult<Vec<DatasourceWithNameResp>> {
        resp_try(async {
            let list = FetcherDatasourceConfigSqlOperate::find_by_platform(
                &db,
                &filter.type_id,
            )
            .await?;
            let resp = list.into_iter().map(Into::into).collect();
            Ok(resp)
        })
        .await
    }
}
