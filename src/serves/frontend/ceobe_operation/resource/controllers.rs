use std::time::Duration;

use database_traits::database_operates::sub_operate::SuperOperate;
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::ceobe_operation::SqlCeobeOperation,
};
use resp_result::{resp_try, FlagWrap};
use tracing::instrument;

use super::{error::FlagResourceRespResult, view::Resource};
use crate::{
    models::sql::resource::operate::ResourceOperate,
    router::CeobeOperationResourceFrontend,
};

impl CeobeOperationResourceFrontend {
    #[instrument(skip(database, modify))]
    pub async fn resource_list(
        mut database: SqlDatabaseOperate,
        mut modify: modify_cache::CheckModify,
    ) -> FlagResourceRespResult<Resource> {
        modify
            .cache_headers
            .get_control()
            .set_max_age(Duration::from_secs(60 * 60));
        resp_try(async {
            let (data, extra) = modify.check_modify(
                database
                    .child::<SqlCeobeOperation<_>>()
                    .child::<ResourceOperate<_>>()
                    .get(|raa, cd| Resource::from((raa, cd)))
                    .await?,
            )?;

            Ok(FlagWrap::new(data, extra))
        })
        .await
    }
}
