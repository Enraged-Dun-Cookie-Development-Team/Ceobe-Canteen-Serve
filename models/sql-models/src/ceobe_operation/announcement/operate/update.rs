use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use sql_connection::database_traits::get_connect::{
    GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
};
use tracing::{info, instrument};

use super::{CeobeOperationAnnouncementSqlOperate, OperateResult};
use crate::ceobe_operation::announcement::{
    checkers::announcement_data::CeobeOpAnnouncement,
    models::model_announcement::{self, ActiveModel},
};
impl CeobeOperationAnnouncementSqlOperate {
    #[instrument(skip(db, announcements), ret)]
    pub async fn update_all<'d, D>(
        db: &'d D, announcements: Vec<CeobeOpAnnouncement>,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + GetDatabaseTransaction + 'd,
        D::Transaction<'d>: ConnectionTrait,
    {
        info!(announcements.update.size = announcements.len());

        let db = db.get_transaction().await?;
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
