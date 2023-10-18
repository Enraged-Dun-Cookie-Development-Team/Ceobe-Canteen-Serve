
use crate::router::CeobeOperationToolLinkFrontend;
use persistence::{mysql::SqlDatabaseOperate, ceobe_operate::models::tool_link::models::model_tool_link::FrontendToolLink};
use super::error::OperateToolLinkRResult;
use ceobe_operation_logic::impletements::CeobeOperateLogic;
use resp_result::resp_try;
use tracing::instrument;

impl CeobeOperationToolLinkFrontend {
    /// 获取工具列表
    #[instrument(ret, skip(sql))]
    pub async fn list(
        sql: SqlDatabaseOperate,
    ) -> OperateToolLinkRResult<Vec<FrontendToolLink>> {
        resp_try(async move {
            Ok(CeobeOperateLogic::find_tool_link_list(sql).await?)
        })
        .await
    }
}