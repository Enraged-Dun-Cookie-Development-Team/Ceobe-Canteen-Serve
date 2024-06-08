use std::time::Duration;

use ceobe_operation_logic::{impletements::CeobeOperateLogic, view::Resource};
use persistence::{
    ceobe_operate::{ToCeobe, ToCeobeOperation},
    mysql::SqlDatabaseOperate,
};
use resp_result::{resp_try, rtry, FlagWrap, RespResult};
use tracing::instrument;

use super::{error::ResourceRespResult};
use crate::router::{CdnOperationResourceFrontend};

impl CdnOperationResourceFrontend {
    
    #[instrument(ret, skip(db))]
    pub async fn resource_list(
        db: SqlDatabaseOperate,
    ) -> ResourceRespResult<Resource> {
        let resp = CeobeOperateLogic::get_resource(db).await;

        RespResult::ok(rtry!(resp))
    }
}
