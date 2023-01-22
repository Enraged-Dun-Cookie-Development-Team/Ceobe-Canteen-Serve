use futures::{Stream, TryStreamExt};
use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait, DbErr,
    EntityTrait, QueryFilter, QueryOrder, StreamTrait,
};
use smallstr::SmallString;
use smallvec::SmallVec;
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::{Tap, TapFallible};
use tracing::{info, instrument};

use super::{AnnouncementOperate, OperateResult};
use crate::{
    ceobe_operation::announcement::models::model_announcement,
    get_zero_data_time,
};

impl<'c, C: 'c> AnnouncementOperate<'c, C> {
    pub async fn find_by_filter_raw(
        filter: impl IntoCondition,
        db: &'c (impl ConnectionTrait + StreamTrait + Send),
    ) -> OperateResult<
        impl Stream<Item = Result<model_announcement::Model, DbErr>> + Send + 'c,
    > {
        Ok(model_announcement::Entity::find()
            .filter(filter)
            .order_by_asc(model_announcement::Column::Order)
            .stream(db)
            .await?)
    }

    pub async fn find_by_filter_not_delete_raw(
        filter: impl IntoCondition,
        db: &'c (impl ConnectionTrait + StreamTrait + Send),
    ) -> OperateResult<
        impl Stream<Item = Result<model_announcement::Model, DbErr>> + Send + 'c,
    > {
        Self::find_by_filter_raw(
            Condition::all().add(filter.into_condition()).add(
                model_announcement::Column::DeleteAt.eq(get_zero_data_time()),
            ),
            db,
        )
        .await
    }
}
impl<'c, C> AnnouncementOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect<'c>: ConnectionTrait + StreamTrait,
{
    #[instrument(skip(self))]
    pub async fn find_all_not_delete(
        &'c self,
    ) -> OperateResult<Vec<model_announcement::Model>> {
        let db = self.get_connect();
        Ok(Self::find_by_filter_not_delete_raw(Condition::all(), db)
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
