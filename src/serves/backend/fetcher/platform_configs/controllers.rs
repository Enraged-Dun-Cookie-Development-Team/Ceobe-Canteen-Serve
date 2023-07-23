use axum::Json;
use checker::CheckExtract;
use persistence::fetcher::{platform_config::ToPlatform, ToFetcher};
use fetcher_logic::{implements::FetcherConfigLogic, view::OneIdReq};
use futures::future;
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::fetcher::platform_config::models::model_platform_config::{
        PlatformBasicInfo, PlatformHasDatasource,
    },
};
use page_size::response::{GenerateListWithPageInfo, ListWithPageInfo};
use resp_result::{resp_try, rtry, MapReject};
use tracing::instrument;

use super::{
    error::PlatformConfigError, FetcherPlatformCheck, PageSizePretreatment,
};
use crate::{
    router::FetcherConfigControllers,
    serves::backend::fetcher::platform_configs::error::PlatformConfigRResult,
};

impl FetcherConfigControllers {
    /// 分页获取平台列表
    // #[instrument(ret, skip(db))]
    pub async fn get_platform_list(
        db: SqlDatabaseOperate, CheckExtract(page_size): PageSizePretreatment,
    ) -> PlatformConfigRResult<ListWithPageInfo<PlatformHasDatasource>> {
        resp_try(async {
            // 获取平台列表
            let platform_list =
                FetcherConfigLogic::get_all_platform_having_datasource_with_paginator(
                    &db, page_size,
                );

                // 并发执行
                let (platform_list, count) =
                future::join(platform_list,
                    // 获取平台数量
                    db.fetcher().platform().count_all()).await;

            let resp = platform_list?.with_page_info(page_size, count?);

            Ok(resp)
        })
        .await
    }

    /// 创建一个平台配置
    #[instrument(ret, skip(db))]
    pub async fn create_platform_config(
        db: SqlDatabaseOperate,
        CheckExtract(platform_config): FetcherPlatformCheck,
    ) -> PlatformConfigRResult<()> {
        rtry!(db.fetcher().platform().create(platform_config).await);
        Ok(()).into()
    }

    /// 更新一个平台配置
    #[instrument(ret, skip(db))]
    pub async fn update_platform_config(
        db: SqlDatabaseOperate,
        CheckExtract(platform_config): FetcherPlatformCheck,
    ) -> PlatformConfigRResult<()> {
        rtry!(db.fetcher().platform().update(platform_config).await);
        Ok(()).into()
    }

    /// 删除平台配置
    #[instrument(ret, skip(db))]
    pub async fn delete_platform_config(
        db: SqlDatabaseOperate,
        MapReject(body): MapReject<Json<OneIdReq>, PlatformConfigError>,
    ) -> PlatformConfigRResult<()> {
        let pid = body.id;
        rtry!(db.fetcher().platform().delete_one(pid).await);
        Ok(()).into()
    }

    #[instrument(skip(db))]
    /// 获取全部平台列表
    pub async fn get_platform_all_list_with_basic_info(
        db: SqlDatabaseOperate,
    ) -> PlatformConfigRResult<Vec<PlatformBasicInfo>> {
        Ok(rtry!(db.fetcher().platform().find_all_basic_info().await)).into()
    }
}
