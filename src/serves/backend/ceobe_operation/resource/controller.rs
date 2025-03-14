use axum_resp_result::{resp_try, rtry, RespResult};
use ceobe_operation_logic::{
    impletements::CeobeOperateLogic, view::Resource,
};
use checker::{CheckExtract, JsonCheckExtract};
use persistence::{
    ceobe_operate::models::resource, mysql::SqlDatabaseOperate,
};
use tencent_cloud_server::cloud_manager::TencentCloudManager;
use tracing::instrument;

use super::error::{ResourceError, ResourceRResult};
use crate::router::CeobeOpResource;

type ResourceUploadCheck = JsonCheckExtract<resource::Checker, ResourceError>;

impl CeobeOpResource {
    #[instrument(ret, skip(db, tc_cloud))]
    pub async fn upload_resource(
        db: SqlDatabaseOperate, tc_cloud: TencentCloudManager,
        CheckExtract(resource): ResourceUploadCheck,
    ) -> ResourceRResult<()> {
        resp_try(async {
            CeobeOperateLogic::upload_resource(db, tc_cloud, resource)
                .await?;
            Ok(())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn get_resource(
        db: SqlDatabaseOperate,
    ) -> ResourceRResult<Resource> {
        let resp = CeobeOperateLogic::get_resource(db).await;

        RespResult::ok(rtry!(resp))
    }
}
