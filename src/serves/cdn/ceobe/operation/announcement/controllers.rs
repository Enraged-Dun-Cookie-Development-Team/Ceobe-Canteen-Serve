use ceobe_operation_logic::{
    impletements::CeobeOperateLogic, view::AnnouncementResp,
};
use persistence::mysql::SqlDatabaseOperate;
use resp_result::resp_try;
use tracing::instrument;

use super::error::AnnouncementRespResult;
use crate::router::CdnOperationAnnouncementFrontend;

impl CdnOperationAnnouncementFrontend {
    // 获取公告列表
    #[instrument(ret, skip(db))]
    pub async fn get_announcement_list(
        db: SqlDatabaseOperate,
    ) -> AnnouncementRespResult<Vec<AnnouncementResp>> {
        resp_try(async {
            Ok(CeobeOperateLogic::get_announcement_list(db).await?)
        })
        .await
    }
}
