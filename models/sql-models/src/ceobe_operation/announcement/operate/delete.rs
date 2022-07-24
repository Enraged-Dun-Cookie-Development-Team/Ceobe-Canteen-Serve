use sea_orm::{ConnectionTrait, EntityTrait, ColumnTrait, Value, sea_query::Expr, QueryFilter};

use crate::ceobe_operation::announcement::models::{get_zero_data_time, model_announcement, get_now_naive_date_time};

use super::{CeoboOperationAnnouncementSqlOperate, OperateResult};

impl CeoboOperationAnnouncementSqlOperate {
    pub async fn all_soft_remove(
        db: &impl ConnectionTrait,
    ) -> OperateResult<u64> {
        let resp = model_announcement::Entity::update_many()
            .filter(model_announcement::Column::DeleteAt.ne(get_zero_data_time()))
            .col_expr(
                model_announcement::Column::DeleteAt,
                Expr::value(Value::ChronoDateTime(get_now_naive_date_time())),
            )
            .exec(db)
            .await?;
        Ok(resp.rows_affected)
    }
}
