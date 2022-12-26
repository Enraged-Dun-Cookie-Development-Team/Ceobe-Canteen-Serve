use crate::serves::backend::fetcher::platform_configs::error::PlatformConfigRResult;
use axum::{debug_handler, Json};
use checker::CheckExtract;
use futures::{future, TryFutureExt};
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::fetcher::platform_config::{
        models::model_platform_config::{PlatformWithHasDatasource, PlatformBasicInfo},
        operate::FetcherPlatformConfigSqlOperate,
    },
};
use page_size::response::{ListWithPageInfo, GenerateListWithPageInfo};
use resp_result::{resp_try, MapReject, rtry};
use tracing::instrument;

use crate::router::FetcherConfigControllers;

use super::{PageSizePretreatment, view::DeleteOnePlatform, error::PlatformConfigError, FetcherPlatformCheck};

impl FetcherConfigControllers {
    #[instrument(ret, skip(db))]
    pub async fn get_platform_list(
        db: SqlConnect, CheckExtract(page_size): PageSizePretreatment,
    ) -> PlatformConfigRResult<ListWithPageInfo<PlatformWithHasDatasource>> {
        resp_try(async {
            // 获取平台列表
            let platform_list = fetcher_logic::implement::get_platform_list_with_has_datasource(&db, page_size);

            // 获取平台数量
            let count = FetcherPlatformConfigSqlOperate::get_platform_total_number(&db);
            // 异步获取
            let (platform_list, count) = future::join(platform_list, count).await;

            let resp = platform_list?.with_page_info(page_size, count?);

            Ok(resp)
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn create_platform_config(
        db: SqlConnect,
        CheckExtract(platform_config): FetcherPlatformCheck
    ) -> PlatformConfigRResult<()> {
        rtry!(FetcherPlatformConfigSqlOperate::create_platform_config(&db, platform_config).await);
        Ok(()).into()
    }

    #[instrument(ret, skip(db))]
    pub async fn update_platform_config(
        db: SqlConnect,
        CheckExtract(platform_config): FetcherPlatformCheck
    ) -> PlatformConfigRResult<()> {
        rtry!(FetcherPlatformConfigSqlOperate::update_platform_config(&db, platform_config).await);
        Ok(()).into()
    }

    #[instrument(ret, skip(db))]
    pub async fn delete_platform_config(
        db: SqlConnect,
        MapReject(body): MapReject<Json<DeleteOnePlatform>, PlatformConfigError>,
    ) -> PlatformConfigRResult<()> {
        let pid = body.id;
        rtry!(FetcherPlatformConfigSqlOperate::delete_one_platform_config(&db, pid).await);
        Ok(()).into()
    }

    #[instrument(skip(db))]
    // 获取全部平台列表
    pub async fn get_platform_all_list_with_basic_info(
        db: SqlConnect
    ) -> PlatformConfigRResult<Vec<PlatformBasicInfo>> {
        Ok(rtry!(FetcherPlatformConfigSqlOperate::find_platform_list_with_basic_info(&db).await)).into()
    }
}
