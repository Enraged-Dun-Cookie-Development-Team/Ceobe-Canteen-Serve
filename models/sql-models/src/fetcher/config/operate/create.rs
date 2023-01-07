use sea_orm::{ConnectionTrait, EntityTrait, IntoActiveModel};
use tracing::instrument;

use super::{FetcherConfigSqlOperate, OperateResult};
use crate::fetcher::config::{
    checkers::config_data::FetcherConfig, models::model_config,
};

impl FetcherConfigSqlOperate {
    #[instrument(skip(ctx, configs))]
    /// 新建单一平台蹲饼配置
    pub async fn create_multi(
        ctx: &impl ConnectionTrait, configs: Vec<FetcherConfig>,
    ) -> OperateResult<()> {
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
