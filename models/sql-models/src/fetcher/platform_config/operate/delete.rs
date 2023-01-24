use sea_orm::{ConnectionTrait, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{info, instrument};

use super::{OperateResult, Platform};
use crate::fetcher::platform_config::{
    models::model_platform_config, operate::OperateError,
};

impl<'c, C> Platform<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect<'c>: ConnectionTrait,
{
    #[instrument(skip(self), ret)]
    /// 删除一个平台
    pub async fn delete_one(&'c self, pid: i32) -> OperateResult<()> {
        info!(platform.id = pid);
        let db = self.get_connect();

        // 获取平台的type，比对数据源表时候有平台的相关数据源
        if !Platform::has_datasource_by_id(db, pid).await? {
            model_platform_config::Entity::delete_by_id(pid)
                .exec(db)
                .await?;
            Ok(())
        } else {
            Err(OperateError::NoDeletePlatformHasDatasource)
        }
    }
}
