use axum::Json;
use resp_result::resp_try;
use tracing::instrument;

use ceobe_operation_logic::impletements::CeobeOperateLogic;
use ceobe_operation_logic::view::{ToolLinkCreateMongoReq, ToolLinkCreateMongoResp};
use persistence::mongodb::MongoDatabaseOperate;
use tencent_cloud_server::cloud_manager::TencentCloudManager;

use crate::router::CdnOperateToolLinkFrontend;

use super::error::CeobeToolLinkRResult;

impl CdnOperateToolLinkFrontend {
    #[instrument(ret, skip(mongo, tc_cloud))]
    pub async fn create_one(
        mongo: MongoDatabaseOperate,
        tc_cloud: TencentCloudManager,
        Json(tool_link): Json<ToolLinkCreateMongoReq>,
    ) -> CeobeToolLinkRResult<()> {
        resp_try(async {
            Ok(CeobeOperateLogic::create_tool_link_mongo(mongo, tc_cloud, tool_link).await.unwrap())
        }).await
    }

    #[instrument(ret, skip(mongo))]
    pub async fn list(
        mongo: MongoDatabaseOperate,
    ) -> CeobeToolLinkRResult<Vec<ToolLinkCreateMongoResp>> {
        resp_try(async {
            Ok(CeobeOperateLogic::list_tool_link_mongo(mongo).await.unwrap())
        }).await
    }
}