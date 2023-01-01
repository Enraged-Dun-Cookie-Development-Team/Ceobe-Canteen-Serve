use axum::Json;
use checker::CheckExtract;
use fetcher_logic::view::OneIdReq;
use futures::future;
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::fetcher::platform_config::{
        models::model_platform_config::{
            PlatformBasicInfo, PlatformWithHasDatasource,
        },
        operate::FetcherPlatformConfigSqlOperate,
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
    // 分页获取平台列表
    #[instrument(ret, skip(db))]
    pub async fn get_platform_list(
        db: SqlConnect, CheckExtract(page_size): PageSizePretreatment,
    ) -> PlatformConfigRResult<ListWithPageInfo<PlatformWithHasDatasource>>
    {
        resp_try(async {
            // 获取平台列表
            let platform_list = fetcher_logic::implement::get_platform_list_with_has_datasource(&db, page_size);

            // 获取平台数量
            let count = FetcherPlatformConfigSqlOperate::get_platform_total_number(&db);
            // 并发执行
            let (platform_list, count) = future::join(platform_list, count).await;

            let resp = platform_list?.with_page_info(page_size, count?);

            Ok(resp)
        })
        .await
    }

    // 创建一个平台配置
    #[instrument(ret, skip(db))]
    pub async fn create_platform_config(
        db: SqlConnect, CheckExtract(platform_config): FetcherPlatformCheck,
    ) -> PlatformConfigRResult<()> {
        rtry!(
            FetcherPlatformConfigSqlOperate::create_platform_config(
                &db,
                platform_config
            )
            .await
        );
        Ok(()).into()
    }

    // 更新一个平台配置
    #[instrument(ret, skip(db))]
    pub async fn update_platform_config(
        db: SqlConnect, CheckExtract(platform_config): FetcherPlatformCheck,
    ) -> PlatformConfigRResult<()> {
        rtry!(
            FetcherPlatformConfigSqlOperate::update_platform_config(
                &db,
                platform_config
            )
            .await
        );
        Ok(()).into()
    }

    // 删除平台配置
    #[instrument(ret, skip(db))]
    pub async fn delete_platform_config(
        db: SqlConnect,
        MapReject(body): MapReject<Json<OneIdReq>, PlatformConfigError>,
    ) -> PlatformConfigRResult<()> {
        let pid = body.id;
        rtry!(
            FetcherPlatformConfigSqlOperate::delete_one_platform_config(
                &db, pid
            )
            .await
        );
        Ok(()).into()
    }

    #[instrument(skip(db))]
    // 获取全部平台列表
    pub async fn get_platform_all_list_with_basic_info(
        db: SqlConnect,
    ) -> PlatformConfigRResult<Vec<PlatformBasicInfo>> {
        Ok(rtry!(FetcherPlatformConfigSqlOperate::find_platform_list_with_basic_info(&db).await)).into()
    }
}
