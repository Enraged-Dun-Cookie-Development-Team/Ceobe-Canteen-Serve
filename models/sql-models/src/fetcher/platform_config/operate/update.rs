use crate::fetcher::platform_config::checkers::platform_config_data::{FetcherPlatformConfigChecker, FetcherPlatformConfig};
use crate::fetcher::platform_config::models::model_platform_config::Model;
use crate::fetcher::platform_config::models::model_platform_config::{
    self, ActiveModel,
};
use crate::fetcher::platform_config::operate::OperateResult;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DbErr, IntoActiveModel, Set,
};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{info, instrument};

use super::FetcherPlatformConfigSqlOperate;

impl FetcherPlatformConfigSqlOperate {
    // 更新平台配置到数据库
    #[instrument(ret, skip(db))]
    pub async fn update_platform_config<'db, D>(
        db: &'db D, config: FetcherPlatformConfig,
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
        let platform_config_active =
            ActiveModel::platform_config_into_active_model(config);
        platform_config_active
            .into_active_model()
            .update(db)
            .await?;

        Ok(())
    }
}
