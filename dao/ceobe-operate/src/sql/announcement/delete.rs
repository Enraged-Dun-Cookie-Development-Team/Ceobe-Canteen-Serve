use db_ops_prelude::{
    get_now_naive_date_time_value, get_zero_data_time,
    sea_orm::{
        sea_query::Expr, ColumnTrait, ConnectionTrait, EntityTrait,
        QueryFilter,
    },
};
use tracing::info;

use super::{AnnouncementOperate, Column, Entity, OperateResult};
impl<C> AnnouncementOperate<'_, C> {
    pub async fn all_soft_remove(
        db: &impl ConnectionTrait,
    ) -> OperateResult<u64> {
        let resp = Entity::update_many()
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
            .col_expr(
                Column::DeleteAt,
                Expr::value(get_now_naive_date_time_value()),
            )
            .exec(db)
            .await?;

        info!(softDelete.effect = resp.rows_affected);
        Ok(resp.rows_affected)
    }
}
