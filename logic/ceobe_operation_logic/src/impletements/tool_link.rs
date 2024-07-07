use futures::future;
use page_size::{
    request::Paginator,
    response::{GenerateListWithPageInfo, ListWithPageInfo},
};
use persistence::{
    bakery::{mansion::ToMansion, ToBakery},
    ceobe_operate::{
        sql_models::tool_link::checkers::tool_link_data::CeobeOperationToolLink,
        ToCeobeOperation,
    },
    ceobe_user::ToCeobe,
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
};
use tencent_cloud_server::{
    cdn::purge_urls_cache::PurgeCachePath, cloud_manager::TencentCloudManager,
};

use super::CeobeOperateLogic;
use crate::{
    error::LogicResult,
    view::{
        OperationTcCdnPath, ToolLinkCreateMongoReq, ToolLinkCreateMongoResp,
        ToolLinkResp, ToolLinkUpdateMongoReq,
    },
};

impl CeobeOperateLogic {
    pub async fn create_tool_link(
        sql: SqlDatabaseOperate, tool_link: CeobeOperationToolLink,
    ) -> LogicResult<()> {
        sql.ceobe()
            .operation()
            .tool_link()
            .create_one(tool_link)
            .await?;
        Ok(())
    }

    pub async fn update_tool_link(
        sql: SqlDatabaseOperate, tool_link: CeobeOperationToolLink,
    ) -> LogicResult<()> {
        sql.ceobe()
            .operation()
            .tool_link()
            .update_one(tool_link)
            .await?;
        Ok(())
    }

    pub async fn delete_tool_link(
        sql: SqlDatabaseOperate, id: i32,
    ) -> LogicResult<()> {
        sql.ceobe().operation().tool_link().delete_one(id).await?;
        Ok(())
    }

    pub async fn find_tool_link_list_with_paginator(
        sql: SqlDatabaseOperate, page_size: Paginator,
    ) -> LogicResult<ListWithPageInfo<ToolLinkResp>> {
        // 获取数据源列表
        // 获取数据源数量
        // 异步获取
        let (tool_list, count) = future::join(
            sql.ceobe()
                .operation()
                .tool_link()
                .find_list_paginator(page_size),
            sql.ceobe()
                .operation()
                .tool_link()
                .get_tool_link_total_number(),
        )
        .await;

        let tool_list = tool_list?;
        let mut tool_links =
            Vec::<ToolLinkResp>::with_capacity(tool_list.len());
        for tool in tool_list {
            tool_links.push(tool.try_into()?)
        }

        let resp = tool_links.with_page_info(page_size, count?);

        Ok(resp)
    }

    pub async fn find_tool_link_list(
        sql: SqlDatabaseOperate,
    ) -> LogicResult<Vec<ToolLinkResp>> {
        let tool_list =
            sql.ceobe().operation().tool_link().find_list().await?;

        let mut tool_links =
            Vec::<ToolLinkResp>::with_capacity(tool_list.len());
        for tool in tool_list {
            tool_links.push(tool.try_into()?)
        }

        Ok(tool_links)
    }

    pub async fn create_tool_link_mongo(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        tool_link: ToolLinkCreateMongoReq,
    ) -> LogicResult<()> {
        mongo
            .ceobe()
            .operation()
            .tool_link_mongo()
            .create(tool_link.into())
            .await
            .unwrap();

        const PATHS: [PurgeCachePath; 1] =
            [OperationTcCdnPath::TOOL_LINK_LIST];
        tc_cloud.purge_urls_cache(&PATHS).await?;

        Ok(())
    }

    pub async fn update_tool_link_mongo(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        tool_link: ToolLinkUpdateMongoReq,
    ) -> LogicResult<()> {
        mongo
            .ceobe()
            .operation()
            .tool_link_mongo()
            .update(tool_link.into())
            .await?;

        const PATHS: [PurgeCachePath; 1] =
            [OperationTcCdnPath::TOOL_LINK_LIST];
        tc_cloud.purge_urls_cache(&PATHS).await?;

        Ok(())
    }

    pub async fn delete_tool_link_mongo(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        id: String,
    ) -> LogicResult<()> {
        mongo
            .ceobe()
            .operation()
            .tool_link_mongo()
            .delete(id)
            .await?;

        const PATHS: [PurgeCachePath; 1] =
            [OperationTcCdnPath::TOOL_LINK_LIST];
        tc_cloud.purge_urls_cache(&PATHS).await?;

        Ok(())
    }

    pub async fn list_tool_link_mongo(
        mongo: MongoDatabaseOperate,
    ) -> LogicResult<Vec<ToolLinkCreateMongoResp>> {
        let tool_link_list =
            mongo.ceobe().operation().tool_link_mongo().list().await?;

        Ok(tool_link_list
            .into_iter()
            .map(|v| v.try_into().unwrap())
            .collect())
    }
}
