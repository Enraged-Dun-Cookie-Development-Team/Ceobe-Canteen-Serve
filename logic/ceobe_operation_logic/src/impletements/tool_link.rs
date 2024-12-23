use futures::future;
use page_size::{
    request::Paginator,
    response::{GenerateListWithPageInfo, ListWithPageInfo},
};
use persistence::{
    ceobe_operate::{
        models::tool_link::checkers::tool_link_data::CeobeOperationToolLink,
        tool_link_mongodb::models::ToolLink, ToCeobe, ToCeobeOperation,
    },
    mongodb::{mongodb::bson, MongoDatabaseOperate},
    mysql::SqlDatabaseOperate,
};
use tencent_cloud_server::{
    cdn::purge_urls_cache::PurgeCachePath, cloud_manager::TencentCloudManager,
};

use super::CeobeOperateLogic;
use crate::{
    error::LogicResult,
    view::{
        OperationTcCdnPath, ToolLinkCreateMongoReq, ToolLinkResp,
        ToolLinkUpdateMongoReq,
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
    ) -> LogicResult<bson::Uuid> {
        let tool_link = ToolLink::from(tool_link);
        let id = tool_link.id;
        mongo
            .ceobe()
            .operation()
            .tool_link()
            .create(tool_link)
            .await
            .unwrap();

        const PATHS: [PurgeCachePath; 1] =
            [OperationTcCdnPath::TOOL_LINK_LIST];
        tc_cloud.purge_urls_cache(&PATHS).await?;

        Ok(id)
    }

    pub async fn update_tool_link_mongo(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        tool_link: ToolLinkUpdateMongoReq,
    ) -> LogicResult<()> {
        mongo
            .ceobe()
            .operation()
            .tool_link()
            .update(tool_link)
            .await?;

        const PATHS: [PurgeCachePath; 1] =
            [OperationTcCdnPath::TOOL_LINK_LIST];
        tc_cloud.purge_urls_cache(&PATHS).await?;

        Ok(())
    }

    pub async fn delete_tool_link_mongo(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        id: bson::Uuid,
    ) -> LogicResult<()> {
        mongo.ceobe().operation().tool_link().delete(id).await?;

        const PATHS: [PurgeCachePath; 1] =
            [OperationTcCdnPath::TOOL_LINK_LIST];
        tc_cloud.purge_urls_cache(&PATHS).await?;

        Ok(())
    }

    pub async fn page_tool_link_mongo(
        mongo: MongoDatabaseOperate, page_size: Paginator,
    ) -> LogicResult<ListWithPageInfo<ToolLink>> {
        let tool_link_list = mongo
            .ceobe()
            .operation()
            .tool_link()
            .all_with_paginator(page_size)
            .await?;

        let count = mongo.ceobe().operation().tool_link().count().await?;

        Ok(tool_link_list.with_page_info(page_size, count))
    }

    pub async fn list_tool_link_mongo(
        mongo: MongoDatabaseOperate,
    ) -> LogicResult<Vec<ToolLink>> {
        let tool_link_list =
            mongo.ceobe().operation().tool_link().all().await?;

        Ok(tool_link_list)
    }
}
