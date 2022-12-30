use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use sql_connection::database_traits::get_connect::{
    GetDatabaseTransaction, TransactionOps,
};
use tracing::{info, instrument};

use super::{FetcherConfigSqlOperate, OperateResult};
use crate::fetcher::config::{
    checkers::config_data::FetcherConfig, models::model_config,
    operate::create::model_config::ActiveModel,
};

impl FetcherConfigSqlOperate {
    #[instrument(skip(db, configs))]
    /// 新建单一平台蹲饼配置
    pub async fn create_configs_by_platform<'db, D>(
        db: &'db D, platform: String, configs: Vec<FetcherConfig>,
    ) -> OperateResult<()>
    where
        D: GetDatabaseTransaction<Error = DbErr> + 'db,
        D::Transaction<'db>: ConnectionTrait,
    {
        info!(fetcherConfig.platform = platform,);

        // model数组转换activeModel数组
        let active_models = configs.into_iter().map(|config| {
            ActiveModel::fetcher_config_into_active_model(config)
        });

        let ctx = db.get_transaction().await?;

        // 删除数据库中该平台的所有配置
        Self::delete_fetcher_configs_by_platform(&ctx, platform).await?;
        // 将该平台的配置写入库中
        model_config::Entity::insert_many(active_models)
            .exec(&ctx)
            .await?;

        ctx.submit().await?;
        Ok(())
    }
}
