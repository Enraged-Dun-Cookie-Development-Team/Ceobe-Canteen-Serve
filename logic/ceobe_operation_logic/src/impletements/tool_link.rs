use checker::LiteChecker;
use futures::future;
use page_size::{
    request::Paginator,
    response::{GenerateListWithPageInfo, ListWithPageInfo},
};
use persistence::{
    ceobe_operate::{
        models::tool_link::{
            self,
            checkers::tool_link_data::{
                CeobeOperationToolLink, CeobeOperationToolLinkUncheck,
                PreCheckCeobeOperationToolLinkChecker,
            },
            models::model_tool_link,
        },
        ToCeobeOperation,
    },
    ceobe_user::ToCeobe,
    mysql::SqlDatabaseOperate,
};

use super::CeobeOperateLogic;
use crate::{
    error::LogicResult,
    view::{ToolLinkBackend, ToolLinkFront},
};

impl CeobeOperateLogic {
    pub async fn create_tool_link(
        sql: SqlDatabaseOperate, tool_link: ToolLinkBackend,
    ) -> LogicResult<()> {
        let tool_link = CeobeOperationToolLinkUncheck::builder()
            .id(tool_link.id)
            .avatar(tool_link.avatar)
            .nickname(tool_link.nickname)
            .jump_url(tool_link.jump_url)
            .slogen(tool_link.slogen)
            .description(tool_link.description)
            .tags(serde_json::to_string(&tool_link.tags)?)
            .build();
        let tool_link =
            PreCheckCeobeOperationToolLinkChecker::lite_check(tool_link)
                .await?;

        sql.ceobe()
            .operation()
            .tool_link()
            .create_one(tool_link)
            .await?;
        Ok(())
    }

    pub async fn update_tool_link(
        sql: SqlDatabaseOperate, tool_link: ToolLinkBackend,
    ) -> LogicResult<()> {
        let tool_link = CeobeOperationToolLinkUncheck::builder()
            .id(tool_link.id)
            .avatar(tool_link.avatar)
            .nickname(tool_link.nickname)
            .jump_url(tool_link.jump_url)
            .slogen(tool_link.slogen)
            .description(tool_link.description)
            .tags(serde_json::to_string(&tool_link.tags)?)
            .build();
        let tool_link =
            PreCheckCeobeOperationToolLinkChecker::lite_check(tool_link)
                .await?;

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
    ) -> LogicResult<ListWithPageInfo<ToolLinkBackend>> {
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
        let mut tool_links = Vec::<ToolLinkBackend>::new();
        for tool in tool_list {
            tool_links.push(ToolLinkBackend::builder()
                .id(Some(tool.id))
                .avatar(tool.avatar)
                .nickname(tool.nickname)
                .jump_url(tool.jump_url)
                .slogen(tool.slogen)
                .description(tool.description)
                .tags(serde_json::from_str::<Vec<String>>(&tool.tags)?)
                .build())
        }
                


        let resp = tool_links.with_page_info(page_size, count?);

        Ok(resp)
    }

    pub async fn find_tool_link_list(
        sql: SqlDatabaseOperate,
    ) -> LogicResult<Vec<ToolLinkFront>> {
        let tool_list = sql.ceobe().operation().tool_link().find_list().await?;

        let mut tool_links = Vec::<ToolLinkFront>::new();
        for tool in tool_list {
            tool_links.push(ToolLinkFront::builder()
                .avatar(tool.avatar)
                .nickname(tool.nickname)
                .jump_url(tool.jump_url)
                .slogen(tool.slogen)
                .description(tool.description)
                .tags(serde_json::from_str::<Vec<String>>(&tool.tags)?)
                .build())
        }

        Ok(tool_links)
    }
}
