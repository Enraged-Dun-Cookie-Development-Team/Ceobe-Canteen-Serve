use sea_orm::{
    sea_query::Expr, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
};

use super::{CeobeOperationAnnouncementSqlOperate, OperateResult};
use crate::{
    ceobe_operation::announcement::models::model_announcement,
    get_now_naive_date_time, get_zero_data_time,
};

impl CeobeOperationAnnouncementSqlOperate {
    pub async fn all_soft_remove(
        db: &impl ConnectionTrait,
    ) -> OperateResult<u64> {
        let resp = model_announcement::Entity::update_many()
            .filter(
                model_announcement::Column::DeleteAt.ne(get_zero_data_time()),
            )
            .col_expr(
                model_announcement::Column::DeleteAt,
                Expr::value(get_now_naive_date_time()),
            )
            .exec(db)
            .await?;
        Ok(resp.rows_affected)
    }
}