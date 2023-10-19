use axum::Json;
use ceobe_operation_logic::{
    impletements::CeobeOperateLogic, view::DeleteOneToolLinkReq,
};
use checker::{CheckExtract, JsonCheckExtract};
use page_size::response::ListWithPageInfo;
use persistence::{
    ceobe_operate::models::tool_link::{
        self, checkers::tool_link_data::PreCheckCeobeOperationToolLinkChecker,
    },
    mysql::SqlDatabaseOperate,
};
use resp_result::{resp_try, MapReject};
use tracing::instrument;

use super::error::{
    OperateToolLinkError, OperateToolLinkRResult, PageSizePretreatment,
};
use crate::router::CeobeOpToolLink;

type CeobeOperationToolLinkCheck = JsonCheckExtract<
    PreCheckCeobeOperationToolLinkChecker,
    OperateToolLinkError,
>;

impl CeobeOpToolLink {
    /// 新增一个工具
    #[instrument(ret, skip(sql))]
    pub async fn create_one(
        sql: SqlDatabaseOperate,
        CheckExtract(tool_link): CeobeOperationToolLinkCheck,
    ) -> OperateToolLinkRResult<()> {
        resp_try(async move {
            CeobeOperateLogic::create_tool_link(sql, tool_link).await?;
            Ok(())
        })
        .await
    }

    /// 更新一个工具
    #[instrument(ret, skip(sql))]
    pub async fn update_one(
        sql: SqlDatabaseOperate,
        CheckExtract(tool_link): CeobeOperationToolLinkCheck,
    ) -> OperateToolLinkRResult<()> {
        resp_try(async move {
            CeobeOperateLogic::update_tool_link(sql, tool_link).await?;
            Ok(())
        })
        .await
    }

    /// 删除一个工具
    #[instrument(ret, skip(sql))]
    pub async fn delete_one(
        sql: SqlDatabaseOperate,
        MapReject(body): MapReject<
            Json<DeleteOneToolLinkReq>,
            OperateToolLinkError,
        >,
    ) -> OperateToolLinkRResult<()> {
        resp_try(async move {
            CeobeOperateLogic::delete_tool_link(sql, body.id).await?;
            Ok(())
        })
        .await
    }

    /// 通过分页获取工具列表
    #[instrument(ret, skip(sql))]
    pub async fn list(
        sql: SqlDatabaseOperate,
        CheckExtract(page_size): PageSizePretreatment,
    ) -> OperateToolLinkRResult<ListWithPageInfo<tool_link::Model>> {
        resp_try(async move {
            Ok(CeobeOperateLogic::find_tool_link_list_with_paginator(
                sql, page_size,
            )
            .await?)
        })
        .await
    }
}
