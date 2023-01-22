use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, CheckExtract,
    JsonCheckExtract,
};

use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::ceobe_operation::{announcement, ToSqlCeobeOperation},
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
        mut db: SqlDatabaseOperate,
    ) -> AnnouncementRespResult<Vec<AnnouncementItem>> {
        resp_try(async {
            Ok(db
                .ceobe_operation()
                .announcement()
                .find_all_not_delete()
                .await?
                .into_iter()
                .map(Into::into)
                .collect())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    // 更新公告列表
    pub async fn update_announcement_list(
        mut db: SqlDatabaseOperate,
        CheckExtract(announcements): UpdateAnnouncementCheck,
    ) -> AnnouncementRespResult<()> {
        resp_try(async {
            db.ceobe_operation()
                .announcement()
                .update_all(announcements)
                .await?;
            Ok(())
        })
        .await
    }
}
