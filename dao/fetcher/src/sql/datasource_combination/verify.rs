use db_ops_prelude::{tap::{Pipe, Tap},ext_traits::select_count::QueryCountByColumn,sql_models::fetcher::datasource_combination::models::model_datasource_combination::{Column, Entity}, sea_orm::{ColumnTrait,ConnectionTrait, EntityTrait, QueryFilter}, get_connect::GetDatabaseConnect};
use tracing::info;

use super::{DatasourceCombinationOperate, OperateResult};

impl<'c, C> DatasourceCombinationOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    /// 数据源组合存在
    pub async fn is_comb_id_exist(
        &self, comb_id: &str,
    ) -> OperateResult<bool> {
        let db = self.get_connect();
        Entity::find()
            .filter(Column::CombinationId.eq(comb_id))
            .count_non_zero_by_column(Column::Id)
            .one(db)
            .await?
            .unwrap()
            .take()
            .tap(|result| {
                info!(
                    comb_id = comb_id,
                    result = if *result { "Exist" } else { "Not Exist" }
                )
            })
            .pipe(Ok)
    }
}
