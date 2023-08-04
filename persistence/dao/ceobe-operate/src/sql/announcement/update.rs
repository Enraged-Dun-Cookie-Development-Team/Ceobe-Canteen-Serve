use db_ops_prelude::{
    get_connect::{GetDatabaseTransaction, TransactionOps},
    sea_orm::{ConnectionTrait, DbErr, EntityTrait},
};
use tracing::{info, instrument};
use db_ops_prelude::ext_traits::with_field::{FieldOrder, With};
use db_ops_prelude::sea_orm::IntoActiveModel;

use super::{AnnouncementOperate, Checked, Entity, OperateResult};

impl<'c, C> AnnouncementOperate<'c, C>
where
    C: GetDatabaseTransaction<Error = DbErr> + 'c,
    C::Transaction<'c>: ConnectionTrait,
{
    #[instrument(skip(self, announcements), ret)]
    pub async fn update_all(
        &'c self, announcements: Vec<Checked>,
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
                  announcement.with(FieldOrder,order as _).into_active_model()
            },
        );
        // 新建数据
        Entity::insert_many(announcement_list).exec(&db).await?;
        db.submit().await?;

        Ok(())
    }
}
