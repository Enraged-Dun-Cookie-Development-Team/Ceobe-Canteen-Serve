use db_ops_prelude::{sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel, StreamTrait}, get_connect::GetDatabaseConnect, sql_models::fetcher::platform_config::checkers::platform_config_data::FetcherPlatformConfig};
use tracing::{info, instrument};

use super::PlatformOperate;
use crate::platform_config::OperateResult;

impl<'c, C> PlatformOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
{
    /// 更新平台配置到数据库
    #[instrument(ret, skip(self))]
    pub async fn update(
        &self, config: FetcherPlatformConfig,
    ) -> OperateResult<()> {
        info!(
            config.id = config.id,
            config.name = config.platform_name,
            config.type_id = config.type_id,
            config.min_request_interval = config.min_request_interval
        );

        let db = self.get_connect();
        config.into_active_model().update(db).await?;

        Ok(())
    }
}
