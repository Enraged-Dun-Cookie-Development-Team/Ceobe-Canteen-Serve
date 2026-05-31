use axum::extract::Query;
use axum_resp_result::resp_try;
use ceobe_operation_logic::impletements::CeobeOperateLogic;
use persistence::{
    ceobe_operate::tool_link_mongodb::models::{ToolLink, ToolLinkKind},
    mongodb::MongoDatabaseOperate,
};
use serde::Deserialize;
use tracing::instrument;

use super::error::CeobeToolLinkRResult;
use crate::router::CdnOperateToolLinkFrontend;

#[derive(Debug, Deserialize)]
pub struct ToolLinkListQuery {
    #[serde(default = "default_kinds")]
    pub kind: Vec<ToolLinkKind>,
}

fn default_kinds() -> Vec<ToolLinkKind> { vec![ToolLinkKind::Arknights] }

impl CdnOperateToolLinkFrontend {
    #[instrument(ret, skip(mongo))]
    pub async fn list(
        mongo: MongoDatabaseOperate, Query(query): Query<ToolLinkListQuery>,
    ) -> CeobeToolLinkRResult<Vec<ToolLink>> {
        resp_try(async {
            Ok(CeobeOperateLogic::list_tool_link_mongo(mongo, query.kind)
                .await?)
        })
        .await
    }
}
