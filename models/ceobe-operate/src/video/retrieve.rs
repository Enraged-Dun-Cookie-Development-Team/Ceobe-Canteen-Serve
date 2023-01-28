use db_ops_prelude::{
    database_operates::NoConnect,
    get_connect::GetDatabaseConnect,
    get_zero_data_time,
    sea_orm::{
        sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait,
        DbErr, EntityTrait, QueryFilter, StreamTrait, QueryOrder,
    },
    tap::TapFallible,
    tracing::instrument, futures::{Stream, TryStreamExt},
};
use tracing::info;

use super::{Model, OperateResult, VideoOperate};

impl VideoOperate<'_, NoConnect> {
    pub async fn find_by_filter_raw<'r, 'db, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<impl Stream<Item = Result<Model, DbErr>> + 'r>
    where
        'db: 'r,
        C: ConnectionTrait + StreamTrait + Send,
    {
        Ok(super::Entity::find()
            .filter(filter)
            .order_by_asc(super::Column::Order)
            .stream(db)
            .await?)
    }

    pub async fn find_by_filter_not_delete_raw<'r, 'db: 'r, C>(
        filter: impl IntoCondition, db: &'db C,
    ) -> OperateResult<impl Stream<Item = Result<Model, DbErr>> + 'r>
    where
        C: ConnectionTrait + StreamTrait + Send,
    {
        let filter = Condition::all()
            .add(filter.into_condition())
            .add(super::Column::DeleteAt.eq(get_zero_data_time()));
        Self::find_by_filter_raw(filter, db).await
    }
}

impl<'s, Conn> VideoOperate<'s, Conn>
where
    Conn: GetDatabaseConnect,
    Conn::Connect: ConnectionTrait + StreamTrait,
{
    #[instrument(skip(self))]
    pub async fn find_all_not_delete(&self) -> OperateResult<Vec<Model>> {
        let db = self.get_connect();

        Ok(VideoOperate::find_by_filter_not_delete_raw(Condition::all(), db)
            .await?
            .try_collect()
            .await?)
        .tap_ok(|list: &Vec<_>| {
            info!(videoList.size = list.len());
        })
    }
}
