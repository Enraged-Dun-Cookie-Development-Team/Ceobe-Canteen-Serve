use ceobe_operation_logic::{
    impletements::CeobeOperateLogic, view::ToolLinkFrontendResp,
};
use persistence::mysql::SqlDatabaseOperate;
use resp_result::resp_try;
use tracing::instrument;

use super::error::OperateToolLinkRResult;
use crate::router::CeobeOperationToolLinkFrontend;

impl CeobeOperationToolLinkFrontend {
    /// 获取工具列表
    #[instrument(ret, skip(sql))]
    pub async fn list(
        sql: SqlDatabaseOperate,
    ) -> OperateToolLinkRResult<Vec<ToolLinkFrontendResp>> {
        resp_try(async move {
            Ok(CeobeOperateLogic::find_tool_link_list(sql).await?)
        })
        .await
    }
}
