use sea_orm::{ConnectionTrait, EntityTrait, QueryFilter, ColumnTrait};
use sea_query::Expr;
use sql_connection::database_traits::database_operates::NoConnect;
use tracing::{info, instrument};

use super::{Datasource, OperateResult};
use crate::{fetcher::datasource_config::{models::model_datasource_config::{Entity, Column}, checkers::FetcherDatasourceConfig}, get_zero_data_time, get_now_naive_date_time_value};


impl Datasource<'_, NoConnect> {
    #[instrument(skip(db), ret)]
    /// 删除一个平台
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
