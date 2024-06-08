use persistence::{ceobe_operate::{announcement, ToCeobeOperation}, ceobe_user::ToCeobe, mysql::SqlDatabaseOperate};

use crate::{error::LogicResult, view::AnnouncementResp};

use super::CeobeOperateLogic;



impl CeobeOperateLogic {
    /// 获取公告列表
    pub async fn get_announcement_list(
        sql: SqlDatabaseOperate
    ) -> LogicResult<Vec<AnnouncementResp>> {
        Ok(sql
            .ceobe()
            .operation()
            .announcement()
            .find_all_not_delete()
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

     /// 更新公告
    pub async fn update_announcement_list(
        sql: SqlDatabaseOperate, announcements: Vec<announcement::Checked>
    ) -> LogicResult<()> {
        sql.ceobe()
            .operation()
            .announcement()
            .update_all(announcements)
            .await?;
        Ok(())
    }
}


