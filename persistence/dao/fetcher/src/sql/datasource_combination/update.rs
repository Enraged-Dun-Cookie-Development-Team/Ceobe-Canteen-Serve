use tracing::info;

use db_ops_prelude::{get_connect::GetDatabaseConnect, get_now_naive_date_time, sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, sea_query::Expr}, sql_models::fetcher::datasource_combination::models::model_datasource_combination::{Column, Entity}};

use super::{DatasourceCombinationOperate, OperateResult};

impl<'c, C> DatasourceCombinationOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    /// 更新数据源组合活跃时间
    pub async fn update_access_time(
        &self, comb_id: &str,
    ) -> OperateResult<()> {
        info!(datasourceComb.combination_id = comb_id);
        let db = self.get_connect();
        Entity::update_many()
            .col_expr(
                Column::LastAccessTime,
                Expr::value(get_now_naive_date_time()),
            )
            .filter(Column::CombinationId.eq(comb_id))
            .exec(db)
            .await?;
        Ok(())
    }
}
