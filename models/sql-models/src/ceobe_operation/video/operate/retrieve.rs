use futures::{Stream, TryStreamExt};
use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait, DbErr,
    EntityTrait, QueryFilter, QueryOrder, StreamTrait,
};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;

use super::{CeobeOperationVideoSqlOperate, OperateResult};
use crate::{
    ceobe_operation::video::models::model_video, get_zero_data_time,
};

impl CeobeOperationVideoSqlOperate {
    pub async fn find_by_filter_raw<'r, 'db, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<
        impl Stream<Item = Result<model_video::Model, DbErr>> + 'r,
    >
    where
        'db: 'r,
        C: ConnectionTrait + StreamTrait<'db> + Send,
    {
        Ok(model_video::Entity::find()
            .filter(filter)
            .order_by_asc(model_video::Column::Order)
            .stream(db)
            .await?)
    }

    pub async fn find_by_filter_not_delete_raw<'r, 'db: 'r, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<
        impl Stream<Item = Result<model_video::Model, DbErr>> + 'r,
    >
    where
        C: ConnectionTrait + StreamTrait<'db> + Send,
    {
        let filter = Condition::all()
            .add(filter.into_condition())
            .add(model_video::Column::DeleteAt.eq(get_zero_data_time()));
        Self::find_by_filter_raw(filter, db).await
    }

    pub async fn find_all_not_delete<'db, D>(
        db: &'db D,
    ) -> OperateResult<Vec<model_video::Model>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        for<'stream> D::Connect<'db>: ConnectionTrait + StreamTrait<'stream>,
    {
        let db = db.get_connect()?;

        Ok(Self::find_by_filter_not_delete_raw(Condition::all(), db)
            .await?
            .try_collect()
            .await?)
    }
}
