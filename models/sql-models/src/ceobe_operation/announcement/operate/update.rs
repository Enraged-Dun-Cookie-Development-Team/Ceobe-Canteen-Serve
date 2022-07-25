use sea_orm::EntityTrait;
use sql_connection::get_sql_transaction;

use super::{CeobeOperationAnnouncementSqlOperate, OperateResult};
use crate::ceobe_operation::announcement::{
    checkers::announcement_data::CeobeOpAnnouncement,
    models::model_announcement::{self, ActiveModel},
};

impl CeobeOperationAnnouncementSqlOperate {
    pub async fn update_all(
        announcements: Vec<CeobeOpAnnouncement>,
    ) -> OperateResult<()> {
        let db = get_sql_transaction().await?;
        // 所有先前的数据都设置为删除
        Self::all_soft_remove(&db).await?;

        // 处理数据，添加order
        let announcement_list = announcements
            .into_iter()
            .enumerate()
            .map(|(order, announcement)| {
                ActiveModel::from_announcement_data_with_order(
                    announcement,
                    order as i32,
                )
            })
            .collect::<Vec<_>>();
        // 新建数据
        model_announcement::Entity::insert_many(announcement_list)
            .exec(&db)
            .await?;
        db.commit().await?;

        Ok(())
    }
}
