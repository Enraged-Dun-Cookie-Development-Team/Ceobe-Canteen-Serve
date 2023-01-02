use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{info, instrument};

use super::{FetcherPlatformConfigSqlOperate, OperateResult};
use crate::fetcher::platform_config::{
    checkers::platform_config_data::FetcherPlatformConfig,
    models::model_platform_config::ActiveModel,
};

impl FetcherPlatformConfigSqlOperate {
    /// 保存平台配置到数据库
    #[instrument(ret, skip(db))]
    pub async fn create_platform_config<'db, D>(
        db: &'db D, config: FetcherPlatformConfig,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            config.name = config.platform_name,
            config.type_id = config.type_id,
            config.min_request_interval = config.min_request_interval
        );
        let db = db.get_connect()?;
        let platform_config_active =
            ActiveModel::platform_config_into_active_model(config);
        platform_config_active.save(db).await?;

        Ok(())
    }
}
