use sea_orm::{
    sea_query::Expr, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
};
use tracing::info;

use super::{CeobeOperationAnnouncementSqlOperate, OperateResult};
use crate::{
    ceobe_operation::announcement::models::model_announcement,
    get_now_naive_date_time_value, get_zero_data_time,
};

impl CeobeOperationAnnouncementSqlOperate {
    pub async fn all_soft_remove(
        db: &impl ConnectionTrait,
    ) -> OperateResult<u64> {
        let resp = model_announcement::Entity::update_many()
            .filter(
                model_announcement::Column::DeleteAt.eq(get_zero_data_time()),
            )
            .col_expr(
                model_announcement::Column::DeleteAt,
                Expr::value(get_now_naive_date_time_value()),
            )
            .exec(db)
            .await?;

        info!(softDelete.effect = resp.rows_affected);
        Ok(resp.rows_affected)
    }
}
