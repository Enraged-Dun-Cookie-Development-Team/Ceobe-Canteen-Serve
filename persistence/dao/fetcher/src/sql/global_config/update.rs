use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{sea_query, ConnectionTrait, EntityTrait, IntoActiveModel},
    sql_models::fetcher::global_config::{
        checkers::global_config_data::FetcherGlobalConfig,
        models::model_global_config::{Column, Entity},
    },
};
use tracing::instrument;

use super::{GlobalOperate, OperateResult};

impl<'c, C> GlobalOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    // 创建或者更新蹲饼器全局配置
    #[instrument(ret, skip_all)]
    pub async fn create_or_update(
        &self, configs: impl IntoIterator<Item = FetcherGlobalConfig>,
    ) -> OperateResult<()> {
        let db = self.get_connect();
        // 转换configs成Vec<ActiveModel>
        let config_list =
            configs.into_iter().map(IntoActiveModel::into_active_model);

        // 存在则更新，不存在则创建
        Entity::insert_many(config_list)
            .on_conflict(
                sea_query::OnConflict::new()
                    .update_column(Column::Value)
                    .to_owned(),
            )
            .exec(db)
            .await?;
        Ok(())
    }
}
