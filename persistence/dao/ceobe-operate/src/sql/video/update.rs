use std::collections::HashMap;

use db_ops_prelude::{
    database_operates::NoConnect,
    futures::{future::ok, stream::iter, StreamExt, TryStreamExt},
    get_connect::{GetDatabaseTransaction, TransactionOps},
    get_now_naive_date_time_value, get_zero_data_time,
    sea_orm::{
        sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait,
        DbErr, EntityTrait, IntoActiveModel, QueryFilter, StreamTrait,
    },
    tap::{Pipe},
};
use tracing::{info, instrument};
use db_ops_prelude::ext_traits::UpdateActiveModel;
use db_ops_prelude::ext_traits::with_field::{FieldOrder, With};

use super::{
     Checked, Column, Entity, OperateResult, VideoOperate,
};

impl VideoOperate<'_, NoConnect> {
    pub async fn all_soft_remove(
        db: &impl ConnectionTrait,
    ) -> OperateResult<u64> {
        let resp = Entity::update_many().filter(Column::DeleteAt.eq(get_zero_data_time())).col_expr(
            Column::DeleteAt,
            Expr::value(get_now_naive_date_time_value()),
        ).exec(db).await?;
        info!(softDelete.effect = resp.rows_affected);
        Ok(resp.rows_affected)
    }
}

impl<'s, Conn> VideoOperate<'s, Conn>
    where Conn: GetDatabaseTransaction<Error=DbErr>,
          Conn::Transaction<'s>: ConnectionTrait + StreamTrait, {
    #[instrument(skip_all, ret, fields(videos.len = videos.len()))]
    pub async fn update_all(
        &'s self, videos: Vec<Checked>,
    ) -> OperateResult<()> {
        let db = self.get_transaction().await?;
        // 所有先前的数据都设置为删除
        VideoOperate::all_soft_remove(&db).await?;

        // 通过BV获取当前已经存在的数据
        let mut exist_data = VideoOperate::find_by_filter_raw(
            Column::Bv.is_in(videos.iter().map(|v| v.bv.as_str())),
            &db,
        ).await?.map_ok(|model| (model.bv.clone(), model)).try_collect::<HashMap<_, _>>().await?;

        info!(existVideos.bv = ?exist_data.keys());

        // 更新或者插入视频信息
        videos.into_iter().enumerate().map(|(order, video)| {
            match exist_data.remove(video.bv.as_str()) {
                Some(model) => {
                    model.into_active_model()
                        .chain_update(video.with(FieldOrder, order as _))
                }
                None => {
                    video.with(FieldOrder, order as _).into_active_model()
                }
            }
        }).pipe(iter).then(|active| active.save(&db)).try_for_each_concurrent(None, |_| ok(())).await?;

        db.submit().await?;
        Ok(())
    }
}
