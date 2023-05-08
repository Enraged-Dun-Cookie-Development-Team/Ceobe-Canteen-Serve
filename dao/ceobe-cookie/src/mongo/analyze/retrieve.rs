use db_ops_prelude::{
    futures::StreamExt,
    mongo_connection::{
        CollectionGuard, MongoDbCollectionTrait, MongoDbError,
    },
    mongodb::{
        bson::{doc, oid::ObjectId},
        options::{FindOneOptions, FindOptions},
    }, mongo_models::ceobe::cookie::analyze::models::{CookieId, AnalyzeModel, CookieInfo},
};
use tracing::instrument;

use super::{OperateResult, AnalyzeOperate};

impl<'db, Conn> AnalyzeOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, AnalyzeModel>,
{
    /// 分页查询饼数据
    #[instrument(skip(self), ret)]
    pub async fn get_data_by_paginate(
        &'db self, first_id: ObjectId, datasources: Vec<i32>,
        page_number: i64,
    ) -> OperateResult<Vec<CookieInfo>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<CookieInfo> =
            &collection.with_mapping();
        let filter = doc! {
            "$and": [
                {
                    "source_config_id": {
                        "$in":datasources
                    }
                },
                {
                    "_id": {
                        "$lte": first_id
                    }
                }
            ]
        };
        let mut vec = collection
            .doing(|collection| {
                collection.find(
                    filter,
                    FindOptions::builder()
                        .projection(doc! {"_id": 0, "meta": 1, "source_config_id": 1, "text": 1, "images": 1, "compress_images": 1, "tags": 1})
                        .sort(doc! {"_id": -1})
                        .limit(page_number)
                        .build(),
                )
            })
            .await?;
        let mut res = Vec::<CookieInfo>::new();
        while let Some(v) = vec.next().await {
            res.push(v.map_err(MongoDbError::from)?);
        }
        Ok(res)
    }

    /// 获取下一页的饼id
    #[instrument(skip(self), ret)]
    pub async fn get_next_page_cookie_id(
        &'db self, first_id: ObjectId, datasources: Vec<i32>,
        page_number: u64,
    ) -> OperateResult<Option<ObjectId>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<CookieId> =
            &collection.with_mapping();
        let filter = doc! {
            "$and": [
                {
                    "source_config_id": {
                        "$in": datasources
                    }
                },
                {
                    "_id": {
                        "$lte":first_id
                    }
                }
            ]
        };
        let cookie_id = collection
            .doing(|collection| {
                collection.find_one(
                    filter,
                    FindOneOptions::builder()
                        .projection(doc! {"_id": 1})
                        .sort(doc! {"_id": -1})
                        .skip(page_number)
                        .build(),
                )
            })
            .await?;

        let res = cookie_id.map(|id| id._id);

        Ok(res)
    }

    /// 获取数据源第一个饼id
    #[instrument(skip(self), ret)]
    pub async fn get_first_cookie_id(
        &'db self, datasources: Vec<i32>,
    ) -> OperateResult<Option<ObjectId>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<CookieId> =
            &collection.with_mapping();
        let filter = doc! {"source_config_id": {"$in":datasources}};
        let cookie_id = collection
            .doing(|collection| {
                collection.find_one(
                    filter,
                    FindOneOptions::builder()
                        .projection(doc! {"_id":1})
                        .sort(doc! {"_id": -1})
                        .build(),
                )
            })
            .await?;

        let res = cookie_id.map(|id| id._id);

        Ok(res)
    }
}
