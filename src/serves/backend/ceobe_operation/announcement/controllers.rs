use axum_prehandle::{prefabs::json::JsonPayload, PreRespHandling, PreHandling};
use checker::prefabs::collect_checkers::iter_checkers::IntoIterChecker;
use orm_migrate::sql_models::ceobe_operation::announcement::{checkers::announcement_data::{CeobeOpAnnouncementUncheck, CeobeOpAnnouncementChecker, CeobeOpAnnouncement}, operate::CeobeOperationAnnouncementSqlOperate};

use crate::utils::data_checker::PreLiteChecker;

use super::{error::{CeobeOperationAnnouncementError, AnnouncementRespResult}, AnnouncementAuthentication, view::AnnouncementItem};

use crate::router::CeobeOperationAnnouncement;


type UpdateAnnouncementCheck = PreLiteChecker<
JsonPayload<Vec<CeobeOpAnnouncementUncheck>>,
IntoIterChecker<
    Vec<CeobeOpAnnouncementUncheck>,
    CeobeOpAnnouncementChecker,
    Vec<CeobeOpAnnouncement>,
>,
CeobeOperationAnnouncementError,
>;

impl CeobeOperationAnnouncement {
    // 获取公告列表
    pub async fn get_announcement_list(_ :AnnouncementAuthentication) 
    -> AnnouncementRespResult<Vec<AnnouncementItem>> {
        Ok(
            CeobeOperationAnnouncementSqlOperate::find_all_not_delete()
                .await?
                .into_iter()
                .map(Into::into)
                .collect()
        ).into()
    }

    // 更新公告列表
    pub async fn update_announcement_list(_ :AnnouncementAuthentication, 
        PreHandling(announcements):PreRespHandling<UpdateAnnouncementCheck>) 
    -> AnnouncementRespResult<()> {
        CeobeOperationAnnouncementSqlOperate::update_all(announcements).await?;
        Ok(()).into()
    }
}