use futures::{Stream, TryStreamExt};
use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait, DbErr,
    EntityTrait, QueryFilter, QueryOrder, StreamTrait,
};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;

use super::{CeobeOperationAnnouncementSqlOperate, OperateResult};
use crate::{
    ceobe_operation::announcement::models::model_announcement,
    get_zero_data_time,
};

impl CeobeOperationAnnouncementSqlOperate {
    pub async fn find_by_filter_raw<'r, 'db, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<
        impl Stream<Item = Result<model_announcement::Model, DbErr>> + Send + 'r,
    >
    where
        'db: 'r,
        C: ConnectionTrait + StreamTrait<'db> + Send,
    {
        Ok(model_announcement::Entity::find()
            .filter(filter)
            .order_by_asc(model_announcement::Column::Order)
            .stream(db)
            .await?)
    }

    pub async fn find_by_filter_not_delete_raw<'r, 'db: 'r, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<
        impl Stream<Item = Result<model_announcement::Model, DbErr>> + Send + 'r,
    >
    where
        C: ConnectionTrait + StreamTrait<'db> + Send,
    {
        Self::find_by_filter_raw(
            Condition::all().add(filter.into_condition()).add(
                model_announcement::Column::DeleteAt.eq(get_zero_data_time()),
            ),
            db,
        )
        .await
    }

    pub async fn find_all_not_delete<'db, D>(
        db: &'db D,
    ) -> OperateResult<Vec<model_announcement::Model>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait + StreamTrait<'db> + Send + 'static,
    {
        Ok(Self::find_by_filter_not_delete_raw(
            Condition::all(),
            db.get_connect()?,
        )
        .await?
        .try_collect()
        .await?)
    }
}
