use page_size::request::PageSize;
use sql_models::{
    fetcher::{
        datasource_config::operate::FetcherDatasourceConfigSqlOperate,
        platform_config::{
            models::model_platform_config::PlatformHasDatasource,
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
    pub async fn get_all_platform_having_datasource_with_paginator<'db, D>(
        db: &'db D, page_size: PageSize,
    ) -> LogicResult<Vec<PlatformHasDatasource>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        // 分页查询平台列表
        let platform_list =
            FetcherPlatformConfigSqlOperate::find_all_with_paginator(
                db, page_size,
            )
            .await?;
        // 获取平台的type的数组
        let platforms = platform_list
            .iter()
            .map(|platform_item| platform_item.type_id.as_str());
        // 查询哪些平台下有数据源
        let platform_has_datasource =
            FetcherDatasourceConfigSqlOperate::any_belong_to_platforms(
                db, platforms,
            )
            .await?;

        let resp = platform_list
            .into_iter()
            .map(|model| {
                (platform_has_datasource.contains(&model.type_id), model)
            })
            .map(|(has_datasource, platform_item)| {
                PlatformHasDatasource::from_model(
                    platform_item,
                    has_datasource,
                )
            })
            .collect();

        Ok(resp)
    }
}
