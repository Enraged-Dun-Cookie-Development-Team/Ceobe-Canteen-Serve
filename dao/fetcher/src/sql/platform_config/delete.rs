use db_ops_prelude::{sea_orm::{ConnectionTrait, EntityTrait, StreamTrait}, get_connect::GetDatabaseConnect, sql_models::fetcher::platform_config::models::model_platform_config::Entity};
use tracing::{info, instrument};

use crate::platform_config::OperateError;

use super::{OperateResult, PlatformOperate};

impl<'c, C> PlatformOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
{
    #[instrument(skip(self), ret)]
    /// 删除一个平台
    pub async fn delete_one(&self, pid: i32) -> OperateResult<()> {
        info!(platform.id = pid);
        let db = self.get_connect();

        // 获取平台的type，比对数据源表时候有平台的相关数据源
        if !PlatformOperate::has_datasource_by_id(db, pid).await? {
            Entity::delete_by_id(pid)
                .exec(db)
                .await?;
            Ok(())
        }
        else {
            Err(OperateError::NoDeletePlatformHasDatasource)
        }
    }
}
