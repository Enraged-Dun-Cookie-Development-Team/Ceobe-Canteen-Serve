use persistence::{mysql::{SqlDatabaseOperate}, ceobe_user::ToCeobe, ceobe_operate::{ToCeobeOperation, models::tool_link::{checkers::tool_link_data::CeobeOperationToolLink, models::model_tool_link::{self, FrontendToolLink}}}};
use page_size::request::Paginator;
use crate::error::LogicResult;

use super::CeobeOperateLogic;

impl CeobeOperateLogic {
    pub async fn create_tool_link(
        sql: SqlDatabaseOperate, tool_link:CeobeOperationToolLink
    ) -> LogicResult<()> {
        sql.ceobe().operation().tool_link().create_one(tool_link).await?;
        Ok(())
    }

    pub async fn update_tool_link(
        sql: SqlDatabaseOperate, tool_link:CeobeOperationToolLink
    ) -> LogicResult<()> {
        sql.ceobe().operation().tool_link().update_one(tool_link).await?;
        Ok(())
    }

    pub async fn delete_tool_link(
        sql: SqlDatabaseOperate, id: i32
    ) -> LogicResult<()> {
        sql.ceobe().operation().tool_link().delete_one(id).await?;
        Ok(())
    }

    pub async fn find_tool_link_list_with_paginator(
        sql: SqlDatabaseOperate, page_size: Paginator,
    ) -> LogicResult<Vec<model_tool_link::Model>> {
        Ok(sql.ceobe().operation().tool_link().find_list_paginator(page_size).await?)
    }

    pub async fn find_tool_link_list(
        sql: SqlDatabaseOperate,
    ) -> LogicResult<Vec<FrontendToolLink>> {
        Ok(sql.ceobe().operation().tool_link().find_list().await?)
    }
}