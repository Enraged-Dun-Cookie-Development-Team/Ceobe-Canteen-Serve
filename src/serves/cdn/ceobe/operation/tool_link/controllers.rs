use axum::{extract::Query, Json};
use ceobe_operation_logic::{
    impletements::CeobeOperateLogic,
    view::{
        ToolLinkCreateMongoReq, ToolLinkCreateMongoResp,
        ToolLinkDeleteMongoReq, ToolLinkUpdateMongoReq,
    },
};
use checker::JsonCheckExtract;
use persistence::{
    ceobe_operate::tool_link_mongodb::ToolLinkChecker,
    mongodb::MongoDatabaseOperate,
};
use resp_result::{resp_try, MapReject};
use tencent_cloud_server::cloud_manager::TencentCloudManager;
use tracing::instrument;

use super::error::{CeobeOperateToolLinkError, CeobeToolLinkRResult};
use crate::router::CdnOperateToolLinkFrontend;

type CreateToolLinkCheck =
    JsonCheckExtract<ToolLinkChecker, CeobeOperateToolLinkError>;

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

    #[instrument(ret, skip(mongo, tc_cloud))]
    pub async fn create_one(
        mongo: MongoDatabaseOperate,
        tc_cloud: TencentCloudManager,
        // CheckExtract(tool_link): CreateToolLinkCheck,
        MapReject(tool_link): MapReject<
            Json<ToolLinkCreateMongoReq>,
            CeobeOperateToolLinkError,
        >,
    ) -> CeobeToolLinkRResult<()> {
        resp_try(async {
            Ok(CeobeOperateLogic::create_tool_link_mongo(
                mongo, tc_cloud, tool_link,
            )
            .await?)
        })
        .await
    }

    #[instrument(ret, skip(mongo, tc_cloud))]
    pub async fn update(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        Json(tool_link): Json<ToolLinkUpdateMongoReq>,
    ) -> CeobeToolLinkRResult<()> {
        resp_try(async {
            Ok(CeobeOperateLogic::update_tool_link_mongo(
                mongo, tc_cloud, tool_link,
            )
            .await?)
        })
        .await
    }

    #[instrument(ret, skip(mongo, tc_cloud))]
    pub async fn delete(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        MapReject(ToolLinkDeleteMongoReq { id }): MapReject<
            Query<ToolLinkDeleteMongoReq>,
            CeobeOperateToolLinkError,
        >,
    ) -> CeobeToolLinkRResult<()> {
        resp_try(async {
            Ok(
                CeobeOperateLogic::delete_tool_link_mongo(
                    mongo, tc_cloud, id,
                )
                .await?,
            )
        })
        .await
    }
}
