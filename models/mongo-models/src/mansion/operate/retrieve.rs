use std::iter::Iterator;

use chrono::{Duration, Local};
use futures::StreamExt;
use mongo_connection::get_mongo_database;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
};

use super::MansionDataMongoOperate;
use crate::mansion::{
    check::Mid,
    preludes::{MansionId, ModelMansion, ModifyAt},
    MansionDataError,
};

impl MansionDataMongoOperate {
    /// 获取单一大厦创建和更新时间
    /// params：mid 大厦id
    pub async fn get_mansion_time_by_id(
        mid: MansionId,
    ) -> Result<ModifyAt, MansionDataError> {
        let db = get_mongo_database();
        let MansionId { main_id, minor_id } = mid;
        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };
        let res = db
            .doing::<_, ModelMansion, _, _>(|collection| {
                async move {
                    let collection = collection.clone_with_type::<ModifyAt>();
                    collection.find_one(filter, None).await
                }
            })
            .await?
            .ok_or(MansionDataError::MansionNotFound)?;
        Ok(res)
    }

    /// 获取单一大厦信息
    /// params：mid 大厦id
    pub async fn get_mansion_by_id(
        mid: MansionId,
    ) -> Result<ModelMansion, MansionDataError> {
        let db = get_mongo_database();
        let MansionId { main_id, minor_id } = mid;
        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };
        let res = db
            .doing::<_, ModelMansion, _, _>(|collection| {
                async move { collection.find_one(filter, None).await }
            })
            .await?
            .ok_or(MansionDataError::MansionNotFound)?;
        Ok(res)
    }

    /// 获取大厦id列表（最底层）
    /// params：filter 过滤器
    pub async fn get_mansion_id_list_by_filter(
        filter: Option<Document>,
    ) -> Result<Vec<String>, MansionDataError> {
        let db = get_mongo_database();

        let res = db
            .doing::<_, ModelMansion, _, _>(|collect| {
                async move {
                    let collect = collect.clone_with_type::<Mid>();
                    let mut vec = collect
                        .find(
                            filter,
                            FindOptions::builder()
                                .projection(doc! {"id":1i32})
                                .sort(doc! {"id.main_id":1,"id.minor_id":1})
                                .build(),
                        )
                        .await?;
                    let mut res = Vec::new();
                    while let Some(v) = vec.next().await {
                        res.push(v?);
                    }
                    Ok(res)
                }
            })
            .await?
            .into_iter()
            .map(|id| id.id.to_string())
            .collect();

        Ok(res)
    }

    /// 无条件获取大厦id列表
    pub async fn get_mansion_id_list() -> Result<Vec<String>, MansionDataError>
    {
        let res = Self::get_mansion_id_list_by_filter(None).await?;
        Ok(res)
    }

    /// 根据时间获取以来的大厦id列表
    /// params： time 往前多少时间
    pub async fn get_mansion_id_list_by_time(
        time: Duration,
    ) -> Result<Vec<String>, MansionDataError> {
        let now = Local::now().naive_local() - time;
        let now =
            mongodb::bson::DateTime::from_millis(now.timestamp_millis());
        let filter = doc! {
            "create_time":{
                "$gte":now
            }
        };

        let res = Self::get_mansion_id_list_by_filter(Some(filter)).await?;
        Ok(res)
    }
}
