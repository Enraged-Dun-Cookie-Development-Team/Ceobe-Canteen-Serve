use ceobe_operation_logic::{impletements::CeobeOperateLogic, view::Resource};
use checker::{CheckExtract, JsonCheckExtract};
use persistence::{
    ceobe_operate::{models::resource, ToCeobe, ToCeobeOperation},
    mysql::SqlDatabaseOperate,
};
use resp_result::{resp_try, rtry, RespResult};
use tracing::instrument;

use super::{
    error::{ResourceError, ResourceRResult},
};
use crate::router::CeobeOpResource;

type ResourceUploadCheck = JsonCheckExtract<resource::Checker, ResourceError>;

impl CeobeOpResource {
    #[instrument(ret, skip(db))]
    pub async fn upload_resource(
        db: SqlDatabaseOperate, CheckExtract(resource): ResourceUploadCheck,
    ) -> ResourceRResult<()> {
        resp_try(async {
            CeobeOperateLogic::upload_resource(db, resource).await?;
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
