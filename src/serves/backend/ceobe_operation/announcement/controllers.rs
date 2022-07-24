use axum_prehandle::{prefabs::json::JsonPayload, PreRespHandling, PreHandling};
use checker::prefabs::collect_checkers::iter_checkers::IntoIterChecker;
use orm_migrate::sql_models::ceobe_operation::announcement::{checkers::announcement_data::{CeobeOpAnnouncementUncheck, CeobeOpAnnouncementChecker, CeobeOpAnnouncement}};

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
    pub async fn get_announcement_list(_ :AnnouncementAuthentication) 
    -> AnnouncementRespResult<Vec<AnnouncementItem>> {

        todo!()
    }

    pub async fn update_announcement_list(_ :AnnouncementAuthentication, 
        PreHandling(announcements):PreRespHandling<UpdateAnnouncementCheck>) 
    -> AnnouncementRespResult<Vec<AnnouncementItem>> {

        todo!()
    }
}