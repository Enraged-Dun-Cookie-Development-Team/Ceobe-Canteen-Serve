use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::{info, instrument};

use super::{FetcherConfigSqlOperate, OperateResult};
use crate::fetcher::config::models::model_config;

impl FetcherConfigSqlOperate {
    #[instrument(skip(db), ret)]
    // 根据平台删除相关配置
    pub async fn delete_fetcher_configs_by_platform<'db>(
        db: &impl ConnectionTrait, platform: String,
    ) -> OperateResult<()> {
        info!(fetcherConfig.platform = platform);

        model_config::Entity::delete_many()
            .filter(model_config::Column::Platform.eq(platform))
            .exec(db)
            .await?;

        Ok(())
    }

    #[instrument(skip(db), ret)]
    // 根据数据源id删除相关配置
    pub async fn delete_fetcher_configs_by_datasource_id<'db>(
        db: &impl ConnectionTrait, did: i32,
    ) -> OperateResult<()> {
        info!(fetcherConfig.did = did);

        model_config::Entity::delete_many()
            .filter(model_config::Column::DatasourceId.eq(did))
            .exec(db)
            .await?;

        Ok(())
    }
}
