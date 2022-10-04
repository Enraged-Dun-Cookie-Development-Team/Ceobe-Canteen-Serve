use modify_cache::CacheMode;
use orm_migrate::{sql_models::ceobe_operation::announcement::operate::CeobeOperationAnnouncementSqlOperate, sql_connection::SqlConnect};
use resp_result::RespResult;

use super::{
    error::FlagAnnouncementRespResult,
    view::{AnnouncementItem, AnnouncementItems},
};
use crate::router::CeobeOperationAnnouncementFrontend;

impl CeobeOperationAnnouncementFrontend {
    // 获取公告列表
    pub async fn get_announcement_list(
        db:SqlConnect,
        mut modify: modify_cache::CheckModify,
    ) -> FlagAnnouncementRespResult<Vec<AnnouncementItem>> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_ty(CacheMode::NoCache);

        let (data, extra) = modify.check_modify(AnnouncementItems(
            CeobeOperationAnnouncementSqlOperate::find_all_not_delete(&db)
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
        ))?;
        RespResult::ok(data)
            .map(AnnouncementItems::into_inner)
            .with_flags(extra)
    }
}
