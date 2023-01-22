use futures::{Stream, TryStreamExt};
use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait, DbErr,
    EntityTrait, QueryFilter, QueryOrder, StreamTrait,
};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::TapFallible;
use tracing::{info, instrument};

use super::{OperateResult, VideoOperate};
use crate::{
    ceobe_operation::video::models::model_video, get_zero_data_time,
};

impl<'c, Conn: 'c> VideoOperate<'c, Conn> {
    pub async fn find_by_filter_raw<'r, 'db, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<
        impl Stream<Item = Result<model_video::Model, DbErr>> + 'r,
    >
    where
        'db: 'r,
        C: ConnectionTrait + StreamTrait + Send,
    {
        Ok(model_video::Entity::find()
            .filter(filter)
            .order_by_asc(model_video::Column::Order)
            .stream(db)
            .await?)
    }

    pub async fn find_by_filter_not_delete_raw<C>(
        filter: impl IntoCondition, db: &'c C,
    ) -> OperateResult<
        impl Stream<Item = Result<model_video::Model, DbErr>> + 'c,
    >
    where
        C: ConnectionTrait + StreamTrait + Send,
    {
        let filter = Condition::all()
            .add(filter.into_condition())
            .add(model_video::Column::DeleteAt.eq(get_zero_data_time()));
        Self::find_by_filter_raw(filter, db).await
    }
}
impl<'c, C> VideoOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect<'c>: ConnectionTrait + StreamTrait,
{
    #[instrument(skip(self))]
    pub async fn find_all_not_delete(
        &'c self,
    ) -> OperateResult<Vec<model_video::Model>> {
        let db = self.get_connect();

        Ok(Self::find_by_filter_not_delete_raw(Condition::all(), db)
            .await?
            .try_collect()
            .await?)
        .tap_ok(|list: &Vec<_>| {
            info!(videoList.size = list.len());
        })
    }
}
