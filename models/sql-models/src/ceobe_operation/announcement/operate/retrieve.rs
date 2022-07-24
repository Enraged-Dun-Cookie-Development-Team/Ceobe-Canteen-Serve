use std::future::ready;

use futures::StreamExt;
use sea_orm::{ConnectionTrait, sea_query::IntoCondition, StreamTrait, Condition, DbErr, EntityTrait, ColumnTrait, QueryFilter, QueryOrder};
use sql_connection::get_sql_database;

use crate::{ceobe_operation::announcement::models::{get_zero_data_time, model_announcement}, StreamResult};

use super::{CeoboOperationAnnouncementSqlOperate, OperateResult};

impl CeoboOperationAnnouncementSqlOperate {
    pub async fn find_by_filter_raw<'r, 'db, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<StreamResult<'r, model_announcement::Model>>
    where
        'db: 'r,
        C: ConnectionTrait + StreamTrait<'db> + Send,
    {
        let v = model_announcement::Entity::find()
            .filter(filter)
            .order_by_asc(model_announcement::Column::Order)
            .stream(db)
            .await?;

        Ok(Box::pin(v))
    }

    pub async fn find_by_filter_not_delete_raw<'r, 'db: 'r, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<StreamResult<'r, model_announcement::Model>>
    where
        C: ConnectionTrait + StreamTrait<'db> + Send,
    {
        let filter = Condition::all()
            .add(filter.into_condition())
            .add(model_announcement::Column::DeleteAt.eq(get_zero_data_time()));
        Self::find_by_filter_raw(filter, db).await
    }

    pub async fn find_all_not_delete(
    ) -> OperateResult<Vec<model_announcement::Model>> {
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