use ceobe_operation_logic::{impletements::CeobeOperateLogic, view::AnnouncementResp};
use modify_cache::CacheMode;
use persistence::{
    ceobe_operate::{ToCeobe, ToCeobeOperation},
    mysql::SqlDatabaseOperate,
};
use resp_result::{resp_try, FlagWrap};
use tracing::instrument;

use crate::router::{CdnOperationAnnouncementFrontend};
use super::error::AnnouncementRespResult;

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
