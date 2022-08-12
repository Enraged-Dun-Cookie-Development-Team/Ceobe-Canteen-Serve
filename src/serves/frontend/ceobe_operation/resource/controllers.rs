use std::time::Duration;

use resp_result::RespResult;

use crate::router::CeobeOperationResourceFrontend;

use crate::models::sql::resource::operate::CeobeOperationResourceSqlOperate;
use super::error::FlagResourceRespResult;
use super::view::Resource;

impl CeobeOperationResourceFrontend{
    pub async fn resource_list(mut modify: modify_cache::CheckModify) -> FlagResourceRespResult<Resource> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        let (data, extra) = modify.check_modify(
            CeobeOperationResourceSqlOperate::get_resource(|raa, cd| {
                Resource::from((raa, cd))
            })
            .await?
        )?;

        RespResult::ok(data).with_flags(extra)
    }
}