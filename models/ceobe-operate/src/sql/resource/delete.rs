use db_ops_prelude::{
    chrono::NaiveDateTime,
    get_zero_data_time,
    sea_orm::{
        sea_query::Expr, ColumnTrait, Condition, ConnectionTrait,
        EntityTrait, QueryFilter,
    },
};
use tracing::info;

use super::{Column, Entity, OperateResult, ResourceOperate, ResourceType};

impl<C> ResourceOperate<'_, C> {
    pub async fn soft_remove(
        db: &impl ConnectionTrait, now: NaiveDateTime, ty: ResourceType,
    ) -> OperateResult<()> {
        let resp = Entity::update_many()
            .filter(
                Condition::all()
                    .add(
                        // set delete for not delete (delete at is timestamp
                        // 0)
                        Column::DeleteAt.eq(get_zero_data_time()),
                    )
                    .add(Column::Ty.eq(ty)),
            )
            .col_expr(Column::DeleteAt, Expr::value(now))
            .exec(db)
            .await?;
        info!(softDelete.effect = resp.rows_affected);
        Ok(())
    }
}
