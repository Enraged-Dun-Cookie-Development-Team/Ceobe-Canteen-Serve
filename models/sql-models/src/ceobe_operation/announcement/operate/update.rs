use sea_orm::EntityTrait;
use sql_connection::get_sql_transaction;

use crate::ceobe_operation::announcement::{checkers::announcement_data::CeobeOperationAnnouncement, models::model_announcement::{ActiveModel, self}};

use super::{CeoboOperationAnnouncementSqlOperate, OperateResult};

impl CeoboOperationAnnouncementSqlOperate {
    pub async fn update_all(announcements :Vec<CeobeOperationAnnouncement>) -> OperateResult<()> {
        let db = get_sql_transaction().await?;
        // 所有先前的数据都设置为删除
        Self::all_soft_remove(&db).await?;

        // 处理数据，添加order
        let mut announcement_list = Vec::new();
        for (order, announcement) in announcements.into_iter().enumerate() {
            let active =  ActiveModel::from_announcement_data_with_order(announcement, order as i32);
            announcement_list.push(active);
        };
        // 新建数据
        model_announcement::Entity::insert_many(announcement_list).exec(&db).await?;

        Ok(())
    }
}