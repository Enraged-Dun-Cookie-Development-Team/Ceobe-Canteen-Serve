use chrono::NaiveDateTime;
use sea_orm::{
    sea_query::Expr, ColumnTrait, Condition, ConnectionTrait, EntityTrait,
    QueryFilter,
};

use super::{CeobeOperationResourceSqlOperate, OperateError};
use crate::{
    ceobe_operation::resource::models::{
        model_resource, resource_type::ResourceType,
    },
    get_zero_data_time,
};

impl CeobeOperationResourceSqlOperate {
    pub async fn soft_remove(
        db: &impl ConnectionTrait, now: NaiveDateTime, ty: ResourceType,
    ) -> Result<(), OperateError> {
        model_resource::Entity::update_many()
            .filter(
                Condition::all()
                    .add(
                        // set delete for not delete (delete at is timestamp
                        // 0)
                        model_resource::Column::DeleteAt
                            .eq(get_zero_data_time()),
                    )
                    .add(model_resource::Column::Ty.eq(ty)),
            )
            .col_expr(model_resource::Column::DeleteAt, Expr::value(now))
            .exec(db)
            .await?;

        Ok(())
    }
}
