use axum::{
    extract::{Multipart, multipart::MultipartRejection, Query},
    Json,
};
use resp_result::{MapReject, resp_try};
use tracing::instrument;

use ceobe_cookie_logic::view::AvatarId;
use ceobe_operation_logic::{
    impletements::CeobeOperateLogic,
    view::{
        DeleteOneToolLinkReq,
        ToolLinkDeleteMongoReq, ToolLinkResp,
    },
};
use ceobe_qiniu_upload::QiniuManager;
use checker::{CheckExtract, JsonCheckExtract};
use page_size::response::ListWithPageInfo;
use persistence::{
    ceobe_operate::tool_link_mongodb::{
        models::ToolLink, ToolLinkChecker,
    },
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
};
use qiniu_cdn_upload::UploadWrap;
use tencent_cloud_server::cloud_manager::TencentCloudManager;

use crate::{
    router::CeobeOpToolLink,
    serves::backend::ceobe_operation::tool_link::{
        error::FieldNotExist, ToolAvatarPayload,
    },
};

use super::error::{
    CeobeOperateToolLinkError, CeobeToolLinkRResult, OperateToolLinkError,
    OperateToolLinkRResult, PageSizePretreatment, ToolLinkPretreatment,
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
    pub async fn all_with_paginator(
        mongo: MongoDatabaseOperate,
        CheckExtract(page_size): PageSizePretreatment,
    ) -> CeobeToolLinkRResult<ListWithPageInfo<ToolLink>> {
        resp_try(async {
            Ok(CeobeOperateLogic::page_tool_link_mongo(mongo, page_size)
                .await?)
        })
        .await
    }

    #[instrument(ret, skip(mongo, tc_cloud))]
    pub async fn create_one_mongo(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        CheckExtract(tool_link): CreateToolLinkCheck,
    ) -> CeobeToolLinkRResult<()> {
        resp_try(async {
            Ok(CeobeOperateLogic::create_tool_link_mongo(
                mongo,
                tc_cloud,
                tool_link.into(),
            )
            .await?)
        })
        .await
    }

    #[instrument(ret, skip(mongo, tc_cloud))]
    pub async fn update_one_mongo(
        mongo: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        CheckExtract(tool_link): CreateToolLinkCheck,
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
