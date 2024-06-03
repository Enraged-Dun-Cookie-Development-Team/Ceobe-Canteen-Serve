use futures::future;
use page_size::{
    request::Paginator,
    response::{GenerateListWithPageInfo, ListWithPageInfo},
};
use persistence::{
    ceobe_operate::{
        models::tool_link::checkers::tool_link_data::CeobeOperationToolLink,
        ToCeobeOperation,
    },
    ceobe_user::ToCeobe,
    mysql::SqlDatabaseOperate,
};

use super::CeobeOperateLogic;
use crate::{error::LogicResult, view::ToolLinkResp};

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

    pub async fn create_tool_link_mongodb() -> () {

    }
}
