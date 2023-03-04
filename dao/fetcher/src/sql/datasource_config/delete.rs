use db_ops_prelude::{sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, sea_query::Expr}, get_zero_data_time, database_operates::NoConnect, get_now_naive_date_time_value, sql_models::fetcher::datasource_config::models::model_datasource_config::Column};
use tracing::{info, instrument};
use db_ops_prelude::sql_models::fetcher::datasource_config::models::model_datasource_config::Entity;
use super::{DatasourceOperate, OperateResult};

impl DatasourceOperate<'_, NoConnect> {
    #[instrument(skip(db), ret)]
    /// 删除一个数据源
    pub async fn delete_one(
        db: &impl ConnectionTrait, did: i32,
    ) -> OperateResult<()> {
        info!(datasource.id = did);
        Entity::update_many()
            .filter(Column::Id.eq(did))
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
            .col_expr(
                Column::DeleteAt,
                Expr::value(get_now_naive_date_time_value()),
            )
            .exec(db)
            .await?;

        Ok(())
    }
}
