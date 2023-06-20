use db_ops_prelude::{
    futures::StreamExt,
    mongo_connection::{
        CollectionGuard, MongoDbCollectionTrait, MongoDbError,
    },
    mongo_models::ceobe::cookie::analyze::models::{
        AnalyzeModel, CookieId, CookieInfo, TerraComicAggregate,
        TerraComicEpisodeInfo,
    },
    mongodb::{
        bson::{self, doc, oid::ObjectId, Bson, Document},
        options::{FindOneOptions, FindOptions},
    },
};
use tracing::instrument;

use super::{AnalyzeOperate, OperateResult};

impl<'db, Conn> AnalyzeOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, AnalyzeModel>,
{
    /// 分页查询饼数据
    #[instrument(skip(self), ret)]
    pub async fn get_data_by_paginate(
        &'db self, first_id: ObjectId, datasources: &[i32], page_size: i64,
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
                        .limit(page_size)
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
        &'db self, first_id: ObjectId, datasources: &[i32], page_size: u64,
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
                        .skip(page_size)
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
        &'db self, datasources: &[i32],
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

    // 获取泰拉记事社漫画各漫画集数量、最后更新时间
    #[instrument(skip(self), ret)]
    pub async fn get_each_terra_comic_count(
        &'db self,
    ) -> OperateResult<Vec<TerraComicAggregate>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<TerraComicAggregate> =
            &collection.with_mapping();
        let mut pipeline = Vec::<Document>::new();
        let group = doc! {
            "$group": {
                "_id": {"comic": "$meta.item.comic"},
                "count": {"$sum": 1},
                "update_time": {"$max": "$meta.timestamp.platform"}
            }
        };
        pipeline.push(group);
        let project = doc! {
            "$project": {
                "_id": 0,
                "comic": "$_id.comic",
                "update_time": "$update_time",
                "count": "$count"
            }
        };
        pipeline.push(project);
        let match_pipeline = doc! {
            "$match": {
                "comic": {"$ne": Bson::Null}
            }
        };
        pipeline.push(match_pipeline);
        let mut vec = collection
            .doing(|collection| collection.aggregate(pipeline, None))
            .await?;
        let mut res = Vec::<TerraComicAggregate>::new();
        while let Some(v) = vec.next().await {
            res.push(bson::from_document(v.map_err(MongoDbError::from)?)?);
        }
        Ok(res)
    }

    /// 获取特定漫画集各章节信息
    #[instrument(skip(self), ret)]
    pub async fn get_terra_comic_episode_list(
        &'db self, comic_id: String,
    ) -> OperateResult<Vec<TerraComicEpisodeInfo>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<TerraComicEpisodeInfo> =
            &collection.with_mapping();
        let filter = doc! {
            "meta.item.comic": comic_id
        };
        let mut vec = collection
            .doing(|collection| {
                collection.find(
                    filter,
                    FindOptions::builder()
                        .projection(doc! {"_id": -1, "comic": "$meta.item.comic", "jump_url": "$meta.item.url", "short_title": "$text"})
                        .sort(doc! {"meta.timestamp.platform": -1})
                        .build(),
                )
            })
            .await?;
        let mut res = Vec::<TerraComicEpisodeInfo>::new();
        while let Some(v) = vec.next().await {
            res.push(v.map_err(MongoDbError::from)?);
        }
        Ok(res)
    }

    /// 获取tag分类饼数量
    #[instrument(skip(self), ret)]
    pub async fn get_tags_cookie_count(
        &'db self, tags: &[&str],
    ) -> OperateResult<u64> {
        let collection = self.get_collection()?;

        let conditions = tags
            .into_iter()
            .map(|tag| doc! {"tags.".to_owned() + tag: {"$exists": true}})
            .collect::<Vec<Document>>();
        let filter = doc! {"$or": conditions};

        let count = collection
            .doing(|collection| collection.count_documents(filter, None))
            .await?;
        Ok(count)
    }

    /// 获取所有饼数量
    #[instrument(skip(self), ret)]
    pub async fn get_cookie_count(&'db self) -> OperateResult<u64> {
        let collection = self.get_collection()?;
        let count = collection
            .doing(|collection| collection.count_documents(None, None))
            .await?;
        Ok(count)
    }
}
