use persistence::{
    ceobe_operate::{announcement, ToCeobe, ToCeobeOperation},
    mysql::SqlDatabaseOperate,
};
use serde::Serialize;
use tencent_cloud_server::{
    cdn::purge_urls_cache::PurgeCachePath, cloud_manager::TencentCloudManager,
};

use super::CeobeOperateLogic;
use crate::{error::LogicResult, view::OperationTcCdnPath};

impl CeobeOperateLogic {
    /// 获取公告列表
    pub async fn get_announcement_list<
        T: Serialize + From<announcement::Model>,
    >(
        sql: SqlDatabaseOperate,
    ) -> LogicResult<Vec<T>> {
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
        sql: SqlDatabaseOperate, tc_cloud: TencentCloudManager,
        announcements: Vec<announcement::Checked>,
    ) -> LogicResult<()> {
        sql.ceobe()
            .operation()
            .announcement()
            .update_all(announcements)
            .await?;

        const PATHS: [PurgeCachePath; 1] =
            [OperationTcCdnPath::ANNOUNCEMENT_LIST_PATH];
        tc_cloud.purge_urls_cache(&PATHS).await?;

        Ok(())
    }
}
