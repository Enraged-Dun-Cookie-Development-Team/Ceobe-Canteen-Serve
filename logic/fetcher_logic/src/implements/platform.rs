use page_size::request::Paginator;
use persistence::{
    fetcher::{
        datasource_config::ToDatasource,
        models::platform_config::models::model_platform_config::PlatformHasDatasource,
        platform_config::ToPlatform, ToFetcher,
    },
    mysql::SqlDatabaseOperate,
};

use crate::{error::LogicResult, implements::FetcherConfigLogic};

impl FetcherConfigLogic {
    /// 分页获取获取平台信息并且附带该平台下有无数据源
    pub async fn get_all_platform_having_datasource_with_paginator(
        db: &SqlDatabaseOperate, page_size: Paginator,
    ) -> LogicResult<Vec<PlatformHasDatasource>> {
        // 分页查询平台列表
        let platform_list = db
            .fetcher()
            .platform()
            .find_all_with_paginator(page_size)
            .await?;
        // 获取平台的type的数组
        let platforms = platform_list
            .iter()
            .map(|platform_item| platform_item.type_id.as_str());
        // 查询哪些平台下有数据源
        let platform_has_datasource = db
            .fetcher()
            .datasource()
            .any_belong_to_platforms(platforms)
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
