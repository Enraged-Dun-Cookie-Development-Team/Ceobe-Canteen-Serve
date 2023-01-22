use sea_orm::{ConnectionTrait, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{info, instrument};

use super::{FetcherPlatformConfigSqlOperate, OperateResult};
use crate::fetcher::platform_config::{
    models::model_platform_config, operate::OperateError,
};

impl FetcherPlatformConfigSqlOperate {
    #[instrument(skip(db), ret)]
    /// 删除一个平台
    pub async fn delete_one<'db, D>(db: &'db D, pid: i32) -> OperateResult<()>
    where
        D: GetDatabaseConnect + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(platform.id = pid);
        let db = db.get_connect();

        // 获取平台的type，比对数据源表时候有平台的相关数据源
        if !Self::has_datasource_by_id(db, pid).await? {
            model_platform_config::Entity::delete_by_id(pid)
                .exec(db)
                .await?;
            Ok(())
        }
        else {
            Err(OperateError::NoDeletePlatformHasDatasource)
        }
    }
}
