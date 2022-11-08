use std::time::Duration;

use orm_migrate::sql_connection::SqlConnect;
use resp_result::{resp_try, FlagWrap};

use super::{error::FlagResourceRespResult, view::Resource};
use crate::{
    models::sql::resource::operate::CeobeOperationResourceSqlOperate,
    router::CeobeOperationResourceFrontend,
};

impl CeobeOperationResourceFrontend {
    pub async fn resource_list(
        db: SqlConnect, mut modify: modify_cache::CheckModify,
    ) -> FlagResourceRespResult<Resource> {
        modify
            .cache_headers
            .get_control()
            .set_max_age(Duration::from_secs(60 * 60));
        resp_try(async {
            let (data, extra) = modify.check_modify(
                CeobeOperationResourceSqlOperate::get_resource(
                    &db,
                    |raa, cd| Resource::from((raa, cd)),
                )
                .await?,
            )?;

            Ok(FlagWrap::new(data, extra))
        })
        .await
    }
}
