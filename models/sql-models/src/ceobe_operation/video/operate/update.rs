use std::collections::HashMap;

use futures::{future::ok, stream::iter, StreamExt, TryStreamExt};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr,
    EntityTrait, IntoActiveModel, QueryFilter, StreamTrait,
};
use sql_connection::database_traits::get_connect::{
    GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
};
use tap::{Pipe, Tap};
use tracing::instrument;

use super::{CeobeOperationVideoSqlOperate, OperateResult};
use crate::{
    ceobe_operation::video::{
        checkers::video_data::CeobeOpVideo,
        models::model_video::{self, ActiveModel},
    },
    get_now_naive_date_time_value, get_zero_data_time,
};

impl CeobeOperationVideoSqlOperate {
    pub async fn all_soft_remove(
        db: &impl ConnectionTrait,
    ) -> OperateResult<u64> {
        let resp = model_video::Entity::update_many()
            .filter(model_video::Column::DeleteAt.eq(get_zero_data_time()))
            .col_expr(
                model_video::Column::DeleteAt,
                Expr::value(get_now_naive_date_time_value()),
            )
            .exec(db)
            .await?;
        Ok(resp.rows_affected)
    }

    #[instrument(skip_all, ret, fields(videos.len = videos.len()))]
    pub async fn update_all<'db, D>(
        db: &'db D, videos: Vec<CeobeOpVideo>,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr>
            + GetDatabaseTransaction
            + 'static,
        for<'stream> D::Transaction<'db>:
            ConnectionTrait + StreamTrait<'stream>,
    {
        let db = db.get_transaction().await?;
        // 所有先前的数据都设置为删除
        Self::all_soft_remove(&db).await?;

        // 通过BV获取当前已经存在的数据
        let mut exist_data = Self::find_by_filter_raw(
            model_video::Column::Bv
                .is_in(videos.iter().map(|v| v.bv.as_str())),
            &db,
        )
        .await?
        .map_ok(|model| (model.bv.clone(), model))
        .try_collect::<HashMap<_, _>>()
        .await?;

        // 更新或者插入视频信息
        videos
            .into_iter()
            .enumerate()
            .map(|(order, video)| {
                match exist_data.remove(video.bv.as_str()) {
                    Some(model) => {
                        model.into_active_model().tap_mut(|active| {
                            active.update_with_video_and_order(
                                video,
                                order as i32,
                            )
                        })
                    }
                    None => ActiveModel::from_video_data_with_order(
                        video,
                        order as i32,
                    ),
                }
            })
            .pipe(iter)
            .then(|active| active.save(&db))
            .try_for_each_concurrent(None, |_| ok(()))
            .await?;

        db.submit().await?;
        Ok(())
    }
}
