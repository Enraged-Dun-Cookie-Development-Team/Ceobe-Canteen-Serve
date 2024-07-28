use ceobe_operation_logic::{
    impletements::CeobeOperateLogic, view::ToolLinkCreateMongoResp,
};
use persistence::mongodb::MongoDatabaseOperate;
use resp_result::resp_try;
use tracing::instrument;

use super::error::CeobeToolLinkRResult;
use crate::router::CdnOperateToolLinkFrontend;

impl CdnOperateToolLinkFrontend {
    #[instrument(ret, skip(mongo))]
    pub async fn list(
        mongo: MongoDatabaseOperate,
    ) -> CeobeToolLinkRResult<Vec<ToolLinkCreateMongoResp>> {
        resp_try(async {
            Ok(CeobeOperateLogic::list_tool_link_mongo(mongo).await?)
        })
        .await
    }
}
