use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use sql_connection::database_traits::get_connect::{
    GetDatabaseTransaction, TransactionOps,
};
use tracing::{info, instrument};

use super::{AnnouncementOperate, OperateResult};
use crate::ceobe_operation::announcement::{
    checkers::announcement_data::CeobeOpAnnouncement,
    models::model_announcement::{self, ActiveModel},
};
impl<'c, C> AnnouncementOperate<'c, C>
where
    C: GetDatabaseTransaction<Error = DbErr> + 'c,
    C::Transaction<'c>: ConnectionTrait,
{
    #[instrument(skip(self, announcements), ret)]
    pub async fn update_all(
        &'c self, announcements: Vec<CeobeOpAnnouncement>,
    ) -> OperateResult<()> {
        info!(announcements.update.size = announcements.len());

        let db = self.get_transaction().await?;
        // 所有先前的数据都设置为删除
        Self::all_soft_remove(&db).await?;
        // 如果为空，直接返回，不需要插入
        if announcements.is_empty() {
            db.submit().await?;
            return Ok(());
        }
        // 处理数据，添加order
        let announcement_list = announcements.into_iter().enumerate().map(
            |(order, announcement)| {
                ActiveModel::from_announcement_data_with_order(
                    announcement,
                    order as i32,
                )
            },
        );
        // 新建数据
        model_announcement::Entity::insert_many(announcement_list)
            .exec(&db)
            .await?;
        db.submit().await?;

        Ok(())
    }
}
