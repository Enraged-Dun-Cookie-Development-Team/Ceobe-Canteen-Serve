use std::time::Duration;

use orm_migrate::sql_connection::SqlConnect;
use resp_result::RespResult;

use super::{
    error::FlagVideoRespResult,
    view::{VideoItem, VideoItems},
};
use crate::{
    models::sql::video::operate::CeobeOperationVideoSqlOperate,
    router::CeobeOperationVideoFrontend,
};

impl CeobeOperationVideoFrontend {
    pub async fn list_all(
        db: SqlConnect, mut modify: modify_cache::CheckModify,
    ) -> FlagVideoRespResult<Vec<VideoItem>> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        let (data, extra) = modify.check_modify(VideoItems(
            CeobeOperationVideoSqlOperate::find_all_not_delete(&db)
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
        ))?;
        RespResult::ok(data)
            .map(VideoItems::into_inner)
            .with_flags(extra)
    }
}
