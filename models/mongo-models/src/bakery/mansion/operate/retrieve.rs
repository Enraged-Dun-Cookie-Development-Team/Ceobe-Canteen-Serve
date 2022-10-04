use std::iter::Iterator;

use chrono::{Duration, Local};
use futures::StreamExt;
use mongo_connection::{CollectionGuard, MongoDbCollectionTrait};
use mongodb::{
    bson::{doc, DateTime, Document},
    options::FindOptions,
};
use tap::Tap;

use super::{MansionDataMongoOperate, OperateError, OperateResult};
use crate::bakery::mansion::{
    checked::Mid,
    preludes::{MansionId, ModelMansion, ModifyAt},
};

impl MansionDataMongoOperate {
    /// 根据条件获取单一大厦创建和更新时间
    /// params：mid 大厦id
    pub async fn get_mansion_time_by_filter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<ModifyAt>,
    ) -> OperateResult<ModifyAt> {
        collection
            .doing(|collection| collection.find_one(filter, None))
            .await?
            .ok_or(OperateError::MansionNotFound)
    }

    /// 获取单一大厦创建和更新时间
    /// params：mid 大厦id
    pub async fn get_mansion_time_by_id(
        mid: &MansionId, collection: &CollectionGuard<ModifyAt>,
    ) -> OperateResult<ModifyAt> {
        Self::get_mansion_time_by_filter(mid.into_id_filter(), collection)
            .await
    }

    /// 获取单一大厦信息
    /// params：mid 大厦id
    pub async fn get_mansion_by_id<'db>(
        db: &'db impl MongoDbCollectionTrait<'db, ModelMansion>,
        mid: &MansionId,
    ) -> OperateResult<ModelMansion> {
        let collection = db.get_collection()?.tap(|c| {
            log::info!("Get MongoDb Collection {:?}", c.namespace())
        });
        collection
            .tap(|c| {
                log::info!(
                    "Start find Mansion Data {:?}, mid = {}",
                    c.namespace(),
                    mid
                )
            })
            .doing(|collection| {
                collection.find_one(mid.into_id_filter(), None)
            })
            .await
            .tap(|re| log::info!("Task  Done is ok :{}", re.is_ok()))?
            .ok_or(OperateError::MansionNotFound)
    }

    /// 获取大厦id列表（最底层）
    /// params：filter 过滤器
    pub async fn get_mansion_id_list_by_filter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<Mid>,
    ) -> OperateResult<Vec<String>> {
        Ok(collection
            .doing(|collection| {
                async move {
                    let mut vec = collection
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
            .collect())
    }

    /// 无条件获取大厦id列表
    pub async fn get_all_mansion_id_list<'db>(
        db: &'db impl MongoDbCollectionTrait<'db, ModelMansion>,
    ) -> OperateResult<Vec<String>> {
        let collection = db.get_collection()?;
        Self::get_mansion_id_list_by_filter(None, &collection.with_mapping())
            .await
    }

    /// 根据时间获取以来的大厦id列表
    /// params： time 往前多少时间
    pub async fn get_mansion_id_list_by_time<'db>(
        db: &'db impl MongoDbCollectionTrait<'db, ModelMansion>,
        time: Duration,
    ) -> OperateResult<Vec<String>> {
        let collection = db.get_collection()?;

        let now = Local::now().naive_local() - time;
        let now = DateTime::from_millis(now.timestamp_millis());
        let filter = doc! {
            "create_time":{
                "$gte":now
            }
        };

        Self::get_mansion_id_list_by_filter(
            filter,
            &collection.with_mapping(),
        )
        .await
    }
}
