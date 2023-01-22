use database_traits::database_operates::sub_operate::SuperOperate;
use modify_cache::CacheMode;
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::ceobe_operation::{
        announcement::operate::AnnouncementOperate, SqlCeobeOperation,
    },
};
use resp_result::{resp_try, FlagWrap};
use tracing::instrument;

use super::{
    error::FlagAnnouncementRespResult,
    view::{AnnouncementItem, AnnouncementItems},
};
use crate::router::CeobeOperationAnnouncementFrontend;

impl CeobeOperationAnnouncementFrontend {
    // 获取公告列表
    #[instrument(skip(db, modify))]
    pub async fn get_announcement_list(
        mut db: SqlDatabaseOperate, mut modify: modify_cache::CheckModify,
    ) -> FlagAnnouncementRespResult<Vec<AnnouncementItem>> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_ty(CacheMode::NoCache);

        resp_try(async {
            let (data, extra) = modify.check_modify(AnnouncementItems(
                db
                .child::<SqlCeobeOperation<_>>()
                    .child::<AnnouncementOperate<_>>()
                    .find_all_not_delete()
                    .await?
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ))?;
            Ok(FlagWrap::new(AnnouncementItems::into_inner(data), extra))
        })
        .await
    }
}
