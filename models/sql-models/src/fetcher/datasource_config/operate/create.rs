use sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel};
use tracing::{info, instrument};

use super::{FetcherDatasourceConfigSqlOperate, OperateResult};
use crate::fetcher::datasource_config::checkers::FetcherDatasourceConfig;

impl FetcherDatasourceConfigSqlOperate {
    /// 保存数据源配置到数据库
    #[instrument(ret, skip(db))]
    pub async fn create(
        db: &impl ConnectionTrait, config: FetcherDatasourceConfig,
    ) -> OperateResult<()> {
        info!(
            datasource.platform = config.platform,
            datasource.datasource = config.datasource,
            datasource.name = config.nickname,
            datasource.avatar = config.avatar.to_string(),
            datasouce.config = ?config.config
        );
        config.into_active_model().save(db).await?;

        Ok(())
    }
}
