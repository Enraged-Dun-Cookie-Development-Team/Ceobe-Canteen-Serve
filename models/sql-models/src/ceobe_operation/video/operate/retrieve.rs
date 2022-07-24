use futures::{future::ready, StreamExt};
use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait, DbErr,
    EntityTrait, QueryFilter, QueryOrder, StreamTrait,
};
use sql_connection::get_sql_database;

use super::{CeoboOperationVideoSqlOperate, OperateResult};
use crate::{ceobe_operation::video::models::model_video, StreamResult, get_zero_data_time};

impl CeoboOperationVideoSqlOperate {
    pub async fn find_by_filter_raw<'r, 'db, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<StreamResult<'r, model_video::Model>>
    where
        'db: 'r,
        C: ConnectionTrait + StreamTrait<'db> + Send,
    {
        let v = model_video::Entity::find()
            .filter(filter)
            .order_by_asc(model_video::Column::Order)
            .stream(db)
            .await?;

        Ok(Box::pin(v))
    }

    pub async fn find_by_filter_not_delete_raw<'r, 'db: 'r, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<StreamResult<'r, model_video::Model>>
    where
        C: ConnectionTrait + StreamTrait<'db> + Send,
    {
        let filter = Condition::all()
            .add(filter.into_condition())
            .add(model_video::Column::DeleteAt.eq(get_zero_data_time()));
        Self::find_by_filter_raw(filter, db).await
    }

    pub async fn find_all_not_delete(
    ) -> OperateResult<Vec<model_video::Model>> {
        let db = get_sql_database();

        Ok(Self::find_by_filter_not_delete_raw(Condition::all(), db)
            .await?
            .fold(Result::<_, DbErr>::Ok(Vec::new()), |vec, data| {
                ready(vec.and_then(|v| Ok((v, data?))).map(
                    |(mut v, data)| {
                        v.push(data);
                        v
                    },
                ))
            })
            .await?)
    }
}
