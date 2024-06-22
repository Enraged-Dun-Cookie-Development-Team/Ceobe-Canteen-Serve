use ceobe_operation_logic::{
    impletements::CeobeOperateLogic, view::AnnouncementResp,
};
use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, CheckExtract,
    JsonCheckExtract,
};
use persistence::{
    ceobe_operate::models::announcement, mysql::SqlDatabaseOperate,
};
use resp_result::resp_try;
use tencent_cloud_server::cloud_manager::TencentCloudManager;
use tracing::instrument;

use super::error::{AnnouncementRespResult, CeobeOperationAnnouncementError};
use crate::router::CeobeOperationAnnouncement;

type UpdateAnnouncementCheck = JsonCheckExtract<
    IntoIterChecker<
        Vec<announcement::Uncheck>,
        announcement::Checker,
        Vec<announcement::Checked>,
    >,
    CeobeOperationAnnouncementError,
>;

impl CeobeOperationAnnouncement {
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

    #[instrument(ret, skip(db, tc_cloud))]
    // 更新公告列表
    pub async fn update_announcement_list(
        db: SqlDatabaseOperate, tc_cloud: TencentCloudManager,
        CheckExtract(announcements): UpdateAnnouncementCheck,
    ) -> AnnouncementRespResult<()> {
        resp_try(async {
            CeobeOperateLogic::update_announcement_list(
                db,
                tc_cloud,
                announcements,
            )
            .await?;
            Ok(())
        })
        .await
    }
}
