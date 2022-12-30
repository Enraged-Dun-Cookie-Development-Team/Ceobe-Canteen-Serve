use sea_orm::{sea_query, ConnectionTrait, DbErr, EntityTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::instrument;

use super::{FetcherGlobalConfigSqlOperate, OperateResult};
use crate::fetcher::global_config::{
    checkers::global_config_data::FetcherGlobalConfig,
    models::{model_global_config, model_global_config::ActiveModel},
};

impl FetcherGlobalConfigSqlOperate {
    // 创建或者更新蹲饼器全局配置
    #[instrument(ret, skip(db))]
    pub async fn create_or_update<'db, D>(
        db: &'db D, configs: Vec<FetcherGlobalConfig>,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        // 转换configs成Vec<ActiveModel>
        let config_list =
            configs.into_iter().enumerate().map(|(_, config)| {
                ActiveModel::global_config_into_active_model(config)
            });

        // 存在则更新，不存在则创建
        model_global_config::Entity::insert_many(config_list)
            .on_conflict(
                sea_query::OnConflict::new()
                    .update_column(model_global_config::Column::Value)
                    .to_owned(),
            )
            .exec(db)
            .await?;
        Ok(())
    }
}
