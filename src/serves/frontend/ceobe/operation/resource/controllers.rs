use std::time::Duration;

use axum_resp_result::{resp_try, FlagWrap};
use persistence::{
    ceobe_operate::{ToCeobe, ToCeobeOperation},
    mysql::SqlDatabaseOperate,
};
use tracing::instrument;

use super::{error::FlagResourceRespResult, view::Resource};
use crate::router::CeobeOperationResourceFrontend;

impl CeobeOperationResourceFrontend {
    #[instrument(skip(database, modify))]
    pub async fn resource_list(
        database: SqlDatabaseOperate, mut modify: modify_cache::CheckModify,
    ) -> FlagResourceRespResult<Resource> {
        modify
            .cache_headers
            .get_control()
            .set_max_age(Duration::from_secs(60 * 60));
        resp_try(async {
            let (data, extra) = modify.check_modify(
                database
                    .ceobe()
                    .operation()
                    .resource()
                    .get(|raa, cd| Resource::from((raa, cd)))
                    .await?,
            )?;

            Ok(FlagWrap::new(data, extra))
        })
        .await
    }
}
