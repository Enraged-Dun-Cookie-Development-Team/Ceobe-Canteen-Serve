use ceobe_operation_logic::{
    impletements::CeobeOperateLogic, view::VideoItem,
};
use persistence::mysql::SqlDatabaseOperate;
use axum_resp_result::resp_try;
use tracing::instrument;

use super::error::VideoRespResult;
use crate::router::CdnOperationVideoFrontend;

impl CdnOperationVideoFrontend {
    #[instrument(ret, skip(database), name = "list all video")]
    pub async fn list_all(
        database: SqlDatabaseOperate,
    ) -> VideoRespResult<Vec<VideoItem>> {
        resp_try(async {
            Ok(CeobeOperateLogic::list_all_video(database).await?)
        })
        .await
    }
}
