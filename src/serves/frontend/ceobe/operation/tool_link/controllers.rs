use ceobe_operation_logic::{
    impletements::CeobeOperateLogic, view::ToolLinkResp,
};
use persistence::mysql::SqlDatabaseOperate;
use axum_resp_result::resp_try;
use tracing::instrument;

use super::error::OperateToolLinkRResult;
use crate::router::CeobeOperationToolLinkFrontend;

impl CeobeOperationToolLinkFrontend {
    /// 获取工具列表
    #[instrument(ret, skip(sql))]
    pub async fn list(
        sql: SqlDatabaseOperate,
    ) -> OperateToolLinkRResult<Vec<ToolLinkResp>> {
        resp_try(async move {
            Ok(CeobeOperateLogic::find_tool_link_list(sql).await?)
        })
        .await
    }
}
