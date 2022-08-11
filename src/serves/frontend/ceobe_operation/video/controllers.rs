use orm_migrate::sql_models::ceobe_operation::video::operate::CeobeOperationVideoSqlOperate;
use resp_result::RespResult;

use super::{error::VideoRespResult, view::VideoItem};
use crate::router::CeobeOperationVideoFrontend;

impl CeobeOperationVideoFrontend {
    pub async fn list_all() -> VideoRespResult<Vec<VideoItem>> {
        RespResult::ok(
            CeobeOperationVideoSqlOperate::find_all_not_delete()
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
        )
    }
}
