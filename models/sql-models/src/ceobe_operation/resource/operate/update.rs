use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ColumnTrait, Condition, ConnectionTrait, EntityTrait,
    QueryFilter,
};
use sql_connection::get_sql_transaction;

use super::{CeobeOperationResourceSqlOperate, OperateError};
use crate::{
    ceobe_operation::resource::{
        checkers::resource_data::CeobeOperationResource,
        models::{model_resource, resource_type::ResourceType},
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

    pub async fn update_resource(
        resource: CeobeOperationResource,
    ) -> Result<(), OperateError> {
        let db = get_sql_transaction().await?;
        let now = Local::now().naive_local();
        // soft remove old resource
        if resource.countdown.is_some() {
            Self::soft_remove(&db, now, ResourceType::Countdown).await?;
        }

        if resource.resource_all_available.is_some() {
            Self::soft_remove(&db, now, ResourceType::ResourceAllAvailable)
                .await?;
        }

        // upload all
        Self::create_new_resource_set(&db, resource, now).await?;

        db.commit().await?;
        Ok(())
    }
}
