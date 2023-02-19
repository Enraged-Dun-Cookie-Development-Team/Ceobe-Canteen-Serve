use sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel};
use sql_connection::database_traits::database_operates::NoConnect;
use tracing::{info, instrument};

use super::{Datasource, OperateResult};
use crate::fetcher::datasource_config::checkers::FetcherDatasourceConfig;

impl Datasource<'_, NoConnect> {
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
        match Self::find_delete_model_by_datasource_and_unique_key(
            db,
            &config.datasource,
            &config.unique_key,
        )
        .await
        {
            Ok(model) => {
                let active_model = model.into_active_model_by_delete(config);
                active_model.update(db).await?;
            }
            Err(_) => {
                config.into_active_model().save(db).await?;
            }
        };

        Ok(())
    }
}
