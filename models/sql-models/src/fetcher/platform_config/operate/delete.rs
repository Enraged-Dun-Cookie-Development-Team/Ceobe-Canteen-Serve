
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{instrument, info};

use crate::fetcher::platform_config::models::model_platform_config;

use super::{FetcherPlatformConfigSqlOperate, OperateResult};

impl FetcherPlatformConfigSqlOperate {
    #[instrument(skip(db), ret)]
    // 删除一个平台
    pub async fn delete_one_platform_config<'db, D>(
        db: &'db D, pid: i32,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(platform.id = pid);
        let db = db.get_connect()?;

        model_platform_config::Entity::delete_by_id(pid).exec(db).await?;

        Ok(())
    }
}
