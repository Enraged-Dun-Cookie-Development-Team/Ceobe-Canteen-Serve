use axum_resp_result::{rtry, RespResult};
use ceobe_operation_logic::{
    impletements::CeobeOperateLogic, view::Resource,
};
use persistence::mysql::SqlDatabaseOperate;
use tracing::instrument;

use super::error::ResourceRespResult;
use crate::router::CdnOperationResourceFrontend;

impl CdnOperationResourceFrontend {
    #[instrument(ret, skip(db))]
    pub async fn resource_list(
        db: SqlDatabaseOperate,
    ) -> ResourceRespResult<Resource> {
        let resp = CeobeOperateLogic::get_resource(db).await;

        RespResult::ok(rtry!(resp))
    }
}
