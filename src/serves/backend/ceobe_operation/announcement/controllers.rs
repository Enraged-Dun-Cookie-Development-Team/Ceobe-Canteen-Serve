use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, CheckExtract,
    JsonCheckExtract,
};
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::ceobe_operation::announcement::{
        checkers::announcement_data::{
            CeobeOpAnnouncement, CeobeOpAnnouncementChecker,
            CeobeOpAnnouncementUncheck,
        },
        operate::CeobeOperationAnnouncementSqlOperate,
    },
};
use resp_result::resp_try;
use tracing::instrument;

use super::{
    error::{AnnouncementRespResult, CeobeOperationAnnouncementError},
    view::AnnouncementItem,
};
use crate::router::CeobeOperationAnnouncement;

type UpdateAnnouncementCheck = JsonCheckExtract<
    IntoIterChecker<
        Vec<CeobeOpAnnouncementUncheck>,
        CeobeOpAnnouncementChecker,
        Vec<CeobeOpAnnouncement>,
    >,
    CeobeOperationAnnouncementError,
>;

impl CeobeOperationAnnouncement {
    // 获取公告列表
    #[instrument(ret, skip(db))]
    pub async fn get_announcement_list(
        db: SqlConnect,
    ) -> AnnouncementRespResult<Vec<AnnouncementItem>> {
        resp_try(async {
            Ok(
                CeobeOperationAnnouncementSqlOperate::find_all_not_delete(
                    &db,
                )
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
            )
        })
        .await
    }

    #[instrument(ret, skip(db))]
    // 更新公告列表
    pub async fn update_announcement_list(
        db: SqlConnect,
        CheckExtract(announcements, _): UpdateAnnouncementCheck,
    ) -> AnnouncementRespResult<()> {
        resp_try(async {
            CeobeOperationAnnouncementSqlOperate::update_all(
                &db,
                announcements,
            )
            .await?;
            Ok(())
        })
        .await
    }
}
