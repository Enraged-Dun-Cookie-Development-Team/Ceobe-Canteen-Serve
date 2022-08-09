use orm_migrate::sql_models::ceobe_operation::announcement::operate::CeobeOperationAnnouncementSqlOperate;

use super::{error::AnnouncementRespResult, view::AnnouncementItem};
use crate::router::CeobeOperationAnnouncementFrontend;

impl CeobeOperationAnnouncementFrontend {
    // 获取公告列表
    pub async fn get_announcement_list(
    ) -> AnnouncementRespResult<Vec<AnnouncementItem>> {
        Ok(CeobeOperationAnnouncementSqlOperate::find_all_not_delete()
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
        .into()
    }
}
