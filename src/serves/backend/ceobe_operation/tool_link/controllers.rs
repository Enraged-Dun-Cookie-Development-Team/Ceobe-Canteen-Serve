use axum::{
    extract::{multipart::MultipartRejection, Multipart, Query},
    Json,
};
use ceobe_cookie_logic::view::AvatarId;
use ceobe_operation_logic::{
    impletements::CeobeOperateLogic,
    view::{
        DeleteOneToolLinkReq, LinkMongoReq, ToolLinkCreateMongoReq,
        ToolLinkCreateMongoResp, ToolLinkDeleteMongoReq, ToolLinkResp,
        ToolLinkUpdateMongoReq,
    },
};
use ceobe_qiniu_upload::QiniuManager;
use checker::{CheckExtract, JsonCheckExtract};
use page_size::response::ListWithPageInfo;
use persistence::{
    ceobe_operate::tool_link_mongodb::{models, Checked, ToolLinkChecker},
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
};
use qiniu_cdn_upload::UploadWrap;
use resp_result::{resp_try, MapReject};
use tencent_cloud_server::cloud_manager::TencentCloudManager;
use tracing::instrument;

use super::error::{
    CeobeOperateToolLinkError, CeobeToolLinkRResult, OperateToolLinkError,
    OperateToolLinkRResult, PageSizePretreatment, ToolLinkPretreatment,
};
use crate::{
    router::CeobeOpToolLink,
    serves::backend::ceobe_operation::tool_link::{
        error::FieldNotExist, ToolAvatarPayload,
    },
};

type CreateToolLinkCheck =
    JsonCheckExtract<ToolLinkChecker, CeobeOperateToolLinkError>;

impl CeobeOpToolLink {
    /// 新增一个工具
    #[instrument(ret, skip(sql))]
    pub async fn create_one(
        sql: SqlDatabaseOperate,
        CheckExtract(tool_link): ToolLinkPretreatment,
    ) -> OperateToolLinkRResult<()> {
        resp_try(async move {
            CeobeOperateLogic::create_tool_link(sql, tool_link).await?;
            Ok(())
        })
        .await
    }

    /// 更新一个工具
    #[instrument(ret, skip(sql))]
    pub async fn update_one(
        sql: SqlDatabaseOperate,
        CheckExtract(tool_link): ToolLinkPretreatment,
    ) -> OperateToolLinkRResult<()> {
        resp_try(async move {
            CeobeOperateLogic::update_tool_link(sql, tool_link).await?;
            Ok(())
        })
        .await
    }

    /// 删除一个工具
    #[instrument(ret, skip(sql))]
    pub async fn delete_one(
        sql: SqlDatabaseOperate,
        MapReject(body): MapReject<
            Json<DeleteOneToolLinkReq>,
            OperateToolLinkError,
        >,
    ) -> OperateToolLinkRResult<()> {
        resp_try(async move {
            CeobeOperateLogic::delete_tool_link(sql, body.id).await?;
            Ok(())
        })
        .await
    }

    /// 通过分页获取工具列表
    #[instrument(ret, skip(sql))]
    pub async fn list(
        sql: SqlDatabaseOperate,
        CheckExtract(page_size): PageSizePretreatment,
    ) -> OperateToolLinkRResult<ListWithPageInfo<ToolLinkResp>> {
        resp_try(async move {
            Ok(CeobeOperateLogic::find_tool_link_list_with_paginator(
                sql, page_size,
            )
            .await?)
        })
        .await
    }

    /// 上传工具头像
    #[instrument(ret, skip(qiniu))]
    pub async fn upload_avatar(
        qiniu: QiniuManager, multipart: Result<Multipart, MultipartRejection>,
    ) -> OperateToolLinkRResult<AvatarId> {
        resp_result::resp_try(async move {
            let mut multipart = multipart?;
            let field = multipart.next_field().await?.ok_or(FieldNotExist)?;

            let resp = qiniu
                .upload(
                    UploadWrap::new(field, ToolAvatarPayload::new()).await?,
                )
                .await
                .map(|resp| AvatarId::from_resp(resp, &qiniu))?;

            Ok(resp)
        })
        .await
    }

    #[instrument(ret, skip(mongo))]
    pub async fn page(
        mongo: MongoDatabaseOperate,
        CheckExtract(page_size): PageSizePretreatment,
    ) -> CeobeToolLinkRResult<ListWithPageInfo<ToolLinkCreateMongoResp>> {
        resp_try(async {
            Ok(CeobeOperateLogic::page_tool_link_mongo(mongo, page_size)
                .await?)
        })
        .await
    }

    #[instrument(ret, skip(mongo, tc_cloud))]
    pub async fn create_one_mongo(
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
    pub async fn update_one_mongo(
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
                mongo,
                tc_cloud,
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
    pub async fn delete_one_mongo(
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
