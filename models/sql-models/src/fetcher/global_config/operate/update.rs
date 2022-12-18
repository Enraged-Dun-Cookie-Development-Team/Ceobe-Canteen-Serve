use sea_orm::{
    sea_query, ConnectionTrait, DbErr, EntityTrait
};
use crate::fetcher::global_config::operate::update::model_global_config::ActiveModel;

use sql_connection::database_traits::get_connect::GetDatabaseConnect;

use crate::fetcher::global_config::{
    checkers::global_config_data::FetcherGlobalConfig,
    models::model_global_config,
};

use super::{FetcherGlobalConfigSqlOperate, OperateResult};

impl FetcherGlobalConfigSqlOperate {
    pub async fn create_or_update<'db, D>(
        db: &'db D, configs: Vec<FetcherGlobalConfig>,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        // 转换configs成Vec<ActiveModel>
        let config_list = configs
            .into_iter()
            .enumerate()
            .map(|(_, config)| {
                ActiveModel::global_config_into_active_model(config)
            })
            .collect::<Vec<_>>();

        // 存在则更新，不存在则创建
        model_global_config::Entity::insert_many(config_list)
            .on_conflict(
                sea_query::OnConflict::new()
                .update_column(
                    model_global_config::Column::Value,
                )
                .to_owned(),
            )
            .exec(db)
            .await?;
        Ok(())
    }
}
