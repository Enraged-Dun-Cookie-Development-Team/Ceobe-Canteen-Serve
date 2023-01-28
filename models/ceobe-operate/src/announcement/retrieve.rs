use db_ops_prelude::{
    database_operates::NoConnect,
    futures::{Stream, TryStreamExt},
    get_connect::GetDatabaseConnect,
    get_zero_data_time,
    sea_orm::{
        sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait,
        DbErr, EntityTrait, QueryFilter, QueryOrder, StreamTrait,
    },
    smallstr::SmallString,
    smallvec::SmallVec,
    tap::{Tap, TapFallible},
};
use tracing::{info, instrument};

use super::{AnnouncementOperate, Column, Entity, Model, OperateResult};

impl AnnouncementOperate<'_, NoConnect> {
    pub async fn find_by_filter_raw<'s, 'db: 's>(
        filter: impl IntoCondition,
        db: &'db (impl ConnectionTrait + StreamTrait + Send + 's),
    ) -> OperateResult<impl Stream<Item = Result<Model, DbErr>> + Send + 's>
    {
        Ok(Entity::find()
            .filter(filter)
            .order_by_asc(Column::Order)
            .stream(db)
            .await?)
    }

    pub async fn find_by_filter_not_delete_raw<'s, 'db: 's>(
        filter: impl IntoCondition,
        db: &'db (impl ConnectionTrait + StreamTrait + Send + 's),
    ) -> OperateResult<impl Stream<Item = Result<Model, DbErr>> + Send + 's>
    {
        Self::find_by_filter_raw(
            Condition::all()
                .add(filter.into_condition())
                .add(Column::DeleteAt.eq(get_zero_data_time())),
            db,
        )
        .await
    }
}
impl<'c, C> AnnouncementOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
{
    #[instrument(skip(self))]
    pub async fn find_all_not_delete(&self) -> OperateResult<Vec<Model>> {
        let db = self.get_connect();
        Ok(AnnouncementOperate::find_by_filter_not_delete_raw(
            Condition::all(),
            db,
        )
        .await?
        .try_collect()
        .await?)
        .tap_ok(|list: &Vec<_>| {
            let contents = list
                .iter()
                .map(|model| &model.content)
                .map(|content| {
                    content
                        .chars()
                        .take(11)
                        .collect::<SmallString<[u8; 14]>>()
                        .tap_mut(|s| {
                            if content.len() > 11 {
                                s.push_str("...")
                            }
                        })
                })
                .collect::<SmallVec<[_; 8]>>();

            info!(list.contents = ?contents);
        })
    }
}
