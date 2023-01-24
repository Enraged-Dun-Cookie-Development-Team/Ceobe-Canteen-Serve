use sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel};
use sql_connection::database_traits::database_operates::NoConnect;
use tracing::{info, instrument};

use super::{Datasource, OperateResult};
use crate::fetcher::datasource_config::checkers::FetcherDatasourceConfig;

impl Datasource<'_,NoConnect> {
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
