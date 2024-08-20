use ceobe_operation_logic::{
    impletements::CeobeOperateLogic,
};
use persistence::mongodb::MongoDatabaseOperate;
use resp_result::resp_try;
use tracing::instrument;
use persistence::ceobe_operate::tool_link_mongodb::models::ToolLink;
use super::error::CeobeToolLinkRResult;
use crate::router::CdnOperateToolLinkFrontend;

impl CdnOperateToolLinkFrontend {
    #[instrument(ret, skip(mongo))]
    pub async fn list(
        mongo: MongoDatabaseOperate,
    ) -> CeobeToolLinkRResult<Vec<ToolLink>> {
        resp_try(async {
            Ok(CeobeOperateLogic::list_tool_link_mongo(mongo).await?)
        })
        .await
    }
}
