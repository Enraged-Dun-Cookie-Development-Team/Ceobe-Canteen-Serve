
use sea_orm::{DbErr, ConnectionTrait, Set, ActiveModelTrait, IntoActiveModel};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{instrument, info};
use crate::fetcher::platform_config::models::model_platform_config::{ActiveModel, self};
use crate::fetcher::platform_config::operate::OperateResult;
use crate::fetcher::platform_config::models::model_platform_config::Model;

use super::FetcherPlatformConfigSqlOperate;

impl FetcherPlatformConfigSqlOperate {
    // 更新平台配置到数据库
    #[instrument(ret, skip(db))]
    pub async fn update_platform_config<'db, D>(
        db: &'db D, config: Model,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            config.id = config.id,
            config.name = config.platform_name,
            config.type_id = config.type_id,
            config.min_request_interval = config.min_request_interval
        );

        let db = db.get_connect()?;
        config.into_active_model().update(db).await?;

        Ok(())
    }
}
