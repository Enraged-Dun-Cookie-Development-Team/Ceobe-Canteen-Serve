use db_ops_prelude::{sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel, StreamTrait}, sql_models::fetcher::platform_config::checkers::platform_config_data::FetcherPlatformConfig, get_connect::GetDatabaseConnect};
use tracing::{info, instrument};

use super::{OperateResult, PlatformOperate};

impl<'c, C> PlatformOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
{
    /// 保存平台配置到数据库
    #[instrument(ret, skip(self))]
    pub async fn create(
        &self, config: FetcherPlatformConfig,
    ) -> OperateResult<()> {
        info!(
            config.name = config.platform_name,
            config.type_id = config.type_id,
            config.min_request_interval = config.min_request_interval
        );
        let db = self.get_connect();
        config.into_active_model().save(db).await?;

        Ok(())
    }
}
