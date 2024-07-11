use axum::{extract::Query};
use ceobe_operation_logic::{
    impletements::CeobeOperateLogic,
    view::{
        LinkMongoReq, ToolLinkCreateMongoReq, ToolLinkCreateMongoResp,
        ToolLinkDeleteMongoReq, ToolLinkUpdateMongoReq,
    },
};
use checker::{CheckExtract, JsonCheckExtract};
use persistence::{
    ceobe_operate::tool_link_mongodb::{models, Checked, ToolLinkChecker},
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
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        CheckExtract(Checked {
            localized_name,
            localized_description,
            localized_slogen,
            localized_tags,
            icon_url,
            links,
            ..
        }): CreateToolLinkCheck,
    ) -> CeobeToolLinkRResult<()> {
        resp_try(async {
            Ok(CeobeOperateLogic::create_tool_link_mongo(
                mongo,
                tc_cloud,
                ToolLinkCreateMongoReq::builder()
                    .localized_name(localized_name)
                    .localized_description(localized_description)
                    .localized_slogen(localized_slogen)
                    .localized_tags(localized_tags)
                    .icon_url(icon_url)
                    .links(
                        links
                            .into_iter()
                            .map(
                                |models::Link {
                                     primary,
                                     regionality,
                                     service,
                                     localized_name,
                                     url,
                                 }| {
                                    LinkMongoReq::builder()
                                        .localized_name(localized_name)
                                        .primary(primary.into())
                                        .regionality(regionality)
                                        .service(service)
                                        .url(url)
                                        .build()
                                },
                            )
                            .collect(),
                    )
                    .build(),
            )
            .await?)
        })
        .await
    }

    #[instrument(ret, skip(mongo, tc_cloud))]
    pub async fn update(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        CheckExtract(Checked {
                         id,
                         localized_name,
                         localized_description,
                         localized_slogen,
                         localized_tags,
                         icon_url,
                         links,
                         ..
                     }): CreateToolLinkCheck,
    ) -> CeobeToolLinkRResult<()> {
        resp_try(async {
            Ok(CeobeOperateLogic::update_tool_link_mongo(
                mongo, tc_cloud,
                ToolLinkUpdateMongoReq::builder()
                    .id(id)
                    .localized_name(localized_name)
                    .localized_description(localized_description)
                    .localized_slogen(localized_slogen)
                    .localized_tags(localized_tags)
                    .icon_url(icon_url)
                    .links(
                        links
                            .into_iter()
                            .map(
                                |models::Link {
                                     primary,
                                     regionality,
                                     service,
                                     localized_name,
                                     url,
                                 }| {
                                    LinkMongoReq::builder()
                                        .localized_name(localized_name)
                                        .primary(primary.into())
                                        .regionality(regionality)
                                        .service(service)
                                        .url(url)
                                        .build()
                                },
                            )
                            .collect(),
                    )
                    .build(),
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
