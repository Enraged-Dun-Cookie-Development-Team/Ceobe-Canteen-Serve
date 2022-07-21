use std::iter::Iterator;

use chrono::{Duration, Local};
use futures::StreamExt;
use mongo_connection::CollectionGuard;
use mongodb::{
    bson::{doc, DateTime, Document},
    options::FindOptions,
};

use super::{get_mansion_collection, MansionDataMongoOperate};
use crate::mansion::{
    checked::Mid,
    preludes::{MansionId, ModelMansion, ModifyAt},
    MansionDataError,
};

impl MansionDataMongoOperate {
    /// 根据条件获取单一大厦创建和更新时间
    /// params：mid 大厦id
    pub async fn get_mansion_time_by_filter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<ModifyAt>,
    ) -> Result<ModifyAt, MansionDataError> {
        Ok(collection
            .doing(|collection| collection.find_one(filter, None))
            .await?
            .ok_or(MansionDataError::MansionNotFound)?)
    }

    /// 获取单一大厦创建和更新时间
    /// params：mid 大厦id
    pub async fn get_mansion_time_by_id(
        mid: &MansionId, collection: &CollectionGuard<ModifyAt>,
    ) -> Result<ModifyAt, MansionDataError> {
        Ok(Self::get_mansion_time_by_filter(mid.into_id_filter(), collection)
            .await?)
    }

    /// 获取单一大厦信息
    /// params：mid 大厦id
    pub async fn get_mansion_by_id(
        mid: &MansionId,
    ) -> Result<ModelMansion, MansionDataError> {
        let collection = get_mansion_collection()?;
        Ok(collection
            .doing(|collection| collection.find_one(mid.into_id_filter(), None))
            .await?
            .ok_or(MansionDataError::MansionNotFound)?)
    }

    /// 获取大厦id列表（最底层）
    /// params：filter 过滤器
    pub async fn get_mansion_id_list_by_filter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<Mid>,
    ) -> Result<Vec<String>, MansionDataError> {
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
    pub async fn get_all_mansion_id_list(
    ) -> Result<Vec<String>, MansionDataError> {
        let collection = get_mansion_collection()?;
        Ok(Self::get_mansion_id_list_by_filter(
            None,
            &collection.with_mapping(),
        )
        .await?)
    }

    /// 根据时间获取以来的大厦id列表
    /// params： time 往前多少时间
    pub async fn get_mansion_id_list_by_time(
        time: Duration,
    ) -> Result<Vec<String>, MansionDataError> {
        let collection = get_mansion_collection()?;

        let now = Local::now().naive_local() - time;
        let now = DateTime::from_millis(now.timestamp_millis());
        let filter = doc! {
            "create_time":{
                "$gte":now
            }
        };

        Ok(Self::get_mansion_id_list_by_filter(
            filter,
            &collection.with_mapping(),
        )
        .await?)
    }
}
