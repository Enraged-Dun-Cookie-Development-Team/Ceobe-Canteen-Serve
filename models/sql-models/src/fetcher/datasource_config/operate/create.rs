use sea_orm::{ActiveModelTrait, ConnectionTrait};
use tracing::{info, instrument};

use super::{FetcherDatasourceConfigSqlOperate, OperateResult};
use crate::fetcher::datasource_config::{
    checkers::datasource_config_data::FetcherDatasourceConfig,
    models::model_datasource_config::ActiveModel,
};

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
        let datasource_config_active =
            ActiveModel::datasource_config_into_active_model(config);
        datasource_config_active.save(db).await?;

        Ok(())
    }
}
