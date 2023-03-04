use db_ops_prelude::{
    database_operates::NoConnect,
    sea_orm::{ConnectionTrait, EntityTrait, IntoActiveModel, StreamTrait},
    sql_models::fetcher::config::{
        checkers::config_data::FetcherConfig, models::model_config,
    },
};
use tracing::instrument;

use super::{ConfigOperate, OperateResult};

impl ConfigOperate<'_, NoConnect> {
    #[instrument(skip(ctx, configs))]
    /// 新建单一平台蹲饼配置
    pub async fn create_multi<'s, 'db, C>(
        ctx: &'db C, configs: Vec<FetcherConfig>,
    ) -> OperateResult<()>
    where
        'db: 's,
        C: ConnectionTrait + StreamTrait + Send,
    {
        // model数组转换activeModel数组
        let active_models =
            configs.into_iter().map(IntoActiveModel::into_active_model);
        // 将该平台的配置写入库中
        model_config::Entity::insert_many(active_models)
            .exec(ctx)
            .await?;

        Ok(())
    }
}
