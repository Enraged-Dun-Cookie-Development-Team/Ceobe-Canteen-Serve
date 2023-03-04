use db_ops_prelude::{sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel, StreamTrait}, database_operates::NoConnect, sql_models::fetcher::datasource_config::checkers::FetcherDatasourceConfig};
use tracing::{info, instrument};

use super::{DatasourceOperate, OperateResult};

impl DatasourceOperate<'_, NoConnect> {
    /// 保存数据源配置到数据库
    #[instrument(ret, skip(db))]
    pub async fn create<'s, 'db, C>(
        db: &'db C, config: FetcherDatasourceConfig,
    ) -> OperateResult<()>
    where
        'db: 's,
        C: ConnectionTrait + StreamTrait + Send,
    {
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
                let active_model = model.recover_active_model(config);
                active_model.update(db).await?;
            }
            Err(_) => {
                config.into_active_model().save(db).await?;
            }
        };

        Ok(())
    }
}
