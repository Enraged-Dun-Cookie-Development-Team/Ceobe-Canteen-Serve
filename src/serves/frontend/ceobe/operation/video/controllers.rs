use std::time::Duration;

use abstract_database::ceobe::ToCeobe;
use ceobe_operate::ToCeobeOperation;
use orm_migrate::sql_connection::SqlDatabaseOperate;
use resp_result::{resp_try, FlagWrap};
use tracing::instrument;

use super::{
    error::FlagVideoRespResult,
    view::{VideoItem, VideoItems},
};
use crate::router::CeobeOperationVideoFrontend;

impl CeobeOperationVideoFrontend {
    #[instrument(skip(db, modify), name = "list all video")]
    pub async fn list_all(
        db: SqlDatabaseOperate, mut modify: modify_cache::CheckModify,
    ) -> FlagVideoRespResult<Vec<VideoItem>> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        resp_try(async {
            let (data, extra) = modify.check_modify(VideoItems(
                db.ceobe()
                    .operation()
                    .video()
                    .find_all_not_delete()
                    .await?
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ))?;
            Ok(FlagWrap::new(VideoItems::into_inner(data), extra))
        })
        .await
    }
}
