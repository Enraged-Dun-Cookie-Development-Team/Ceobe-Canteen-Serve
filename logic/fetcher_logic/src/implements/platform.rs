use page_size::request::PageSize;
use sql_models::{
    fetcher::{
        datasource_config::operate::FetcherDatasourceConfigSqlOperate,
        platform_config::{
            models::model_platform_config::PlatformWithHasDatasource,
            operate::FetcherPlatformConfigSqlOperate,
        },
    },
    sql_connection::{
        database_traits::get_connect::GetDatabaseConnect,
        sea_orm::{ConnectionTrait, DbErr},
    },
};

use crate::{error::LogicResult, implements::FetcherConfigLogic};

impl FetcherConfigLogic {
    /// 分页获取获取平台信息并且附带该平台下有无数据源
    pub async fn get_platform_list_with_has_datasource<'db, D>(
        db: &'db D, page_size: PageSize,
    ) -> LogicResult<Vec<PlatformWithHasDatasource>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        // 分页查询平台列表
        let platform_list =
            FetcherPlatformConfigSqlOperate::find_platform_list_by_page_size(
                db, page_size,
            )
            .await?;
        // 获取平台的type的数组
        let platforms = platform_list
            .iter()
            .map(|platform_item| platform_item.type_id.clone())
            .collect();
        // 查询哪些平台下有数据源
        let platform_datasource_exist_map =
            FetcherDatasourceConfigSqlOperate::has_datasource_from_platforms(
                db, platforms,
            )
            .await?;

        let resp = platform_list
            .into_iter()
            .map(|platform_item| {
                PlatformWithHasDatasource {
                    id: platform_item.id,
                    type_id: platform_item.type_id.clone(),
                    platform_name: platform_item.platform_name,
                    min_request_interval: platform_item.min_request_interval,
                    has_datasource: platform_datasource_exist_map
                        .contains(&platform_item.type_id),
                }
            })
            .collect();

        Ok(resp)
    }
}
