use db_ops_prelude::{
    database_operates::NoConnect,
    sea_orm::{
        ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, StreamTrait,
    },
    sql_models::fetcher::config::models::model_config,
};
use tracing::{info, instrument};

use super::{ConfigOperate, OperateResult};

impl ConfigOperate<'_, NoConnect> {
    #[instrument(skip(db), ret)]
    /// 根据平台删除相关配置
    pub async fn delete_by_platform<'s, 'db, C>(
        db: &'db C, platform: &str,
    ) -> OperateResult<()>
    where
        'db: 's,
        C: ConnectionTrait + StreamTrait + Send,
    {
        info!(fetcherConfig.platform = platform);

        model_config::Entity::delete_many()
            .filter(model_config::Column::Platform.eq(platform))
            .exec(db)
            .await?;

        Ok(())
    }

    #[instrument(skip(db), ret)]
    /// 根据数据源id删除相关配置
    pub async fn delete_by_datasource_id<'s, 'db, C>(
        db: &'db C, did: i32,
    ) -> OperateResult<()>
    where
        'db: 's,
        C: ConnectionTrait + StreamTrait + Send,
    {
        info!(fetcherConfig.did = did);

        model_config::Entity::delete_many()
            .filter(model_config::Column::DatasourceId.eq(did))
            .exec(db)
            .await?;

        Ok(())
    }
}
