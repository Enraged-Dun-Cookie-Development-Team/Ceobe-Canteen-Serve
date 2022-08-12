use std::time::Duration;

use resp_result::RespResult;

use super::{error::FlagResourceRespResult, view::Resource};
use crate::{
    models::sql::resource::operate::CeobeOperationResourceSqlOperate,
    router::CeobeOperationResourceFrontend,
};

impl CeobeOperationResourceFrontend {
    pub async fn resource_list(
        mut modify: modify_cache::CheckModify,
    ) -> FlagResourceRespResult<Resource> {
        modify.cache_headers.get_control()
            .set_max_age(Duration::from_secs(60 * 60));

        let (data, extra) = modify.check_modify(
            CeobeOperationResourceSqlOperate::get_resource(|raa, cd| {
                Resource::from((raa, cd))
            })
            .await?,
        )?;

        RespResult::ok(data).with_flags(extra)
    }
}
