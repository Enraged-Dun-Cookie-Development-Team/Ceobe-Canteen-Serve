use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::{info, instrument};

use super::{FetcherConfigSqlOperate, OperateResult};
use crate::fetcher::config::models::model_config;

impl FetcherConfigSqlOperate {
    #[instrument(skip(db), ret)]
    /// 根据平台删除相关配置
    pub async fn delete_by_platform(
        db: &impl ConnectionTrait, platform: &str,
    ) -> OperateResult<()> {
        info!(fetcherConfig.platform = platform);

        model_config::Entity::delete_many()
            .filter(model_config::Column::Platform.eq(platform))
            .exec(db)
            .await?;

        Ok(())
    }

    /// 根据平台删除相关配置,但是有多个平台
    pub async fn delete_by_all_platform(
        db: &impl ConnectionTrait, platforms: impl IntoIterator<Item = &str>,
    ) -> OperateResult<()> {
        model_config::Entity::delete_many()
            .filter(model_config::Column::Platform.is_in(platforms))
            .exec(db)
            .await?;

        Ok(())
    }

    #[instrument(skip(db), ret)]
    /// 根据数据源id删除相关配置
    pub async fn delete_by_datasource_id(
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
