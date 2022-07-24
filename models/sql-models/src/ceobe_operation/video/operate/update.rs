use std::collections::HashMap;

use futures::{future::ready, StreamExt};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait,
    EntityTrait, IntoActiveModel, QueryFilter, Value,
};
use sql_connection::get_sql_transaction;

use super::{CeoboOperationVideoSqlOperate, OperateResult};
use crate::ceobe_operation::video::{
    checkers::video_data::CeoboOpVideo,
    models::{
        get_now_naive_date_time, get_zero_data_time,
        model_video::{self, ActiveModel},
    },
};

impl CeoboOperationVideoSqlOperate {
    pub async fn all_soft_remove(
        db: &impl ConnectionTrait,
    ) -> OperateResult<u64> {
        let resp = model_video::Entity::update_many()
            .filter(model_video::Column::DeleteAt.ne(get_zero_data_time()))
            .col_expr(
                model_video::Column::DeleteAt,
                Expr::value(Value::ChronoDateTime(get_now_naive_date_time())),
            )
            .exec(db)
            .await?;
        Ok(resp.rows_affected)
    }

    pub async fn update_all(videos: Vec<CeoboOpVideo>) -> OperateResult<()> {
        let db = get_sql_transaction().await?;
        // 所有先前的数据都设置为删除
        Self::all_soft_remove(&db).await?;

        // 通过BV获取当前已经存在的数据
        let mut exist_data = Self::find_by_filter_raw(
            model_video::Column::Bv
                .is_in(videos.iter().map(|v| v.bv.as_str())),
            &db,
        )
        .await?
        .fold(Ok(HashMap::new()), |map, data| {
            ready({
                data.and_then(|data| Ok((data, map?))).map(
                    |(data, mut map)| {
                        map.insert(data.bv.clone(), data);
                        map
                    },
                )
            })
        })
        .await?;

        // 更新或者插入视频信息
        for active in videos.into_iter().enumerate().map(|(order, video)| {
            if let Some(model) = exist_data.remove(video.bv.as_str()) {
                let mut active = model.into_active_model();
                active.update_with_video_and_order(video, order as i32);
                active
            }
            else {
                ActiveModel::from_video_data_with_order(video, order as i32)
            }
        }) {
            active.save(&db).await?;
        }

        db.commit().await?;
        Ok(())
    }
}
