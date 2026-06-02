use axum::extract::Query;
use axum_resp_result::resp_try;
use ceobe_operation_logic::{impletements::CeobeOperateLogic, view::ToolLinkListReq};
use persistence::{
    ceobe_operate::tool_link_mongodb::models::ToolLink,
    mongodb::MongoDatabaseOperate,
};
use tracing::instrument;

use super::error::CeobeToolLinkRResult;
use crate::router::CdnOperateToolLinkFrontend;

impl CdnOperateToolLinkFrontend {
    #[instrument(ret, skip(mongo))]
    pub async fn list(
        mongo: MongoDatabaseOperate, Query(query): Query<ToolLinkListReq>,
    ) -> CeobeToolLinkRResult<Vec<ToolLink>> {
        resp_try(async {
            Ok(CeobeOperateLogic::list_tool_link_mongo(mongo, query.kind)
                .await?)
        })
        .await
    }
}
