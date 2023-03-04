use db_ops_prelude::{sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter}, sql_models::fetcher::config::models::model_config, database_operates::NoConnect};
use tracing::{info, instrument};

use super::{ConfigOperate, OperateResult};

impl ConfigOperate<'_, NoConnect> {
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
