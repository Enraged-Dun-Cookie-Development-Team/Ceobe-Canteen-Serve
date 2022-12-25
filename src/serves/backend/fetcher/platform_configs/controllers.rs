use crate::serves::backend::fetcher::platform_configs::error::PlatformConfigRResult;
use axum::debug_handler;
use checker::CheckExtract;
use futures::{future, TryFutureExt};
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::fetcher::platform_config::{
        models::model_platform_config::PlatformWithHasDatasource,
        operate::FetcherPlatformConfigSqlOperate,
    },
};
use page_size::response::{ListWithPageInfo, GenerateListWithPageInfo};
use resp_result::resp_try;
use tracing::instrument;

use crate::router::FetcherConfigControllers;

use super::PageSizePretreatment;

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

    // #[instrument(ret, skip(db))]
    // pub fn create_platform_config(
    //     db: SqlConnect,
    // ) -> PlatformConfigRResult<()> {

    // }

    // #[instrument(ret, skip(db))]
    // pub fn update_platform_config(
    //     db: SqlConnect,
    // ) -> PlatformConfigRResult<()> {

    // }

    // #[instrument(ret, skip(db))]
    // pub fn delete_platform_config(
    //     db: SqlConnect,
    // ) -> PlatformConfigRResult<()> {
        
    // }
}
