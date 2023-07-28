use db_ops_prelude::{
    futures::StreamExt,
    mongo_connection::{
        CollectionGuard, MongoDbCollectionTrait, MongoDbError,
    },
    mongo_models::ceobe::cookie::analyze::models::{
        AnalyzeModel, CookieId, CookieInfo, CookieInfoWithId,
        CookieSimpleInfo, TerraComicAggregate, TerraComicEpisodeInfo,
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
                        .projection(doc! {
                            "_id": 0,
                            "meta": 1,
                            "source_config_id": 1,
                            "text": 1,
                            "images": 1,
                            "compress_images": 1,
                            "tags": 1
                        })
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
        let sort = doc! {
            "$sort": {
                "update_time": -1
            }
        };
        pipeline.push(sort);
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
                        .projection(doc! {
                            "_id": -1,
                            "comic":
                            "$meta.item.comic",
                            "jump_url": "$meta.item.url",
                            "short_title": "$text"
                        })
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

    /// 根据object_id获取相应的饼
    #[instrument(skip(self), ret)]
    pub async fn get_data_by_object_ids(
        &'db self, object_ids: &[ObjectId],
    ) -> OperateResult<Vec<CookieInfoWithId>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<CookieInfoWithId> =
            &collection.with_mapping();
        let filter = doc! {
            "_id": {
                "$in": object_ids
            }
        };
        let mut vec = collection
            .doing(|collection| {
                collection.find(
                    filter,
                    FindOptions::builder()
                        .projection(doc! {
                            "_id": 1,
                            "meta": 1,
                            "source_config_id": 1,
                            "text": 1,
                            "images": 1,
                            "tags": 1
                        })
                        .sort(doc! {"_id": -1})
                        .build(),
                )
            })
            .await?;
        let mut res = Vec::<CookieInfoWithId>::new();
        while let Some(v) = vec.next().await {
            res.push(v.map_err(MongoDbError::from)?);
        }
        Ok(res)
    }

    /// 根据keywords分页查询饼数据
    pub async fn get_data_by_paginate_and_keyword(
        &'db self, first_id: Option<ObjectId>, datasources: &[i32],
        page_size: i64, keyword: &str,
    ) -> OperateResult<Vec<CookieInfo>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<CookieInfo> =
            &collection.with_mapping();
        let filter = match first_id {
            Some(id) => {
                doc! {
                    "$and": [
                        {
                            "source_config_id": {
                                "$in":datasources
                            }
                        },
                        {
                            "_id": {
                                "$lte": id
                            }
                        },
                        {
                            "$or": [
                                {
                                    format!("keywords.{keyword}"): {
                                        "$exists": true
                                    }
                                },
                                {
                                    format!("tags.{keyword}"): {
                                        "$exists": true
                                    }
                                }
                            ]
                            
                        }
                    ]
                }
            }
            None => {
                doc! {
                    "$and": [
                        {
                            "source_config_id": {
                                "$in":datasources
                            }
                        },
                        {
                            format!("keywords.{keyword}"): {
                                "$exists": true
                            }
                        }
                    ]
                }
            }
        };
        let mut vec = collection
            .doing(|collection| {
                collection.find(
                    filter,
                    FindOptions::builder()
                        .projection(doc! {
                            "_id": 0,
                            "meta": 1,
                            "source_config_id": 1,
                            "text": 1,
                            "images": 1,
                            "tags": 1
                        })
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

    /// 根据keywords获取下一页的饼id
    #[instrument(skip(self), ret)]
    pub async fn get_next_page_cookie_id_by_keyword(
        &'db self, first_id: Option<ObjectId>, datasources: &[i32],
        page_size: u64, keyword: &str,
    ) -> OperateResult<Option<ObjectId>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<CookieId> =
            &collection.with_mapping();
        let filter = match first_id {
            Some(id) => {
                doc! {
                    "$and": [
                        {
                            "source_config_id": {
                                "$in":datasources
                            }
                        },
                        {
                            "_id": {
                                "$lte": id
                            }
                        },
                        {
                            "keywords.".to_string() + keyword: {
                                "$exists": true
                            }
                        }
                    ]
                }
            }
            None => {
                doc! {
                    "$and": [
                        {
                            "source_config_id": {
                                "$in":datasources
                            }
                        },
                        {
                            "keywords.".to_string() + keyword: {
                                "$exists": true
                            }
                        }
                    ]
                }
            }
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

    /// 获取tag分类饼数量
    #[instrument(skip(self), ret)]
    pub async fn get_cookie_count_by_tags(
        &'db self, tags: &[&str],
    ) -> OperateResult<u64> {
        let collection = self.get_collection()?;

        let conditions = tags
            .iter()
            .map(|tag| doc! {format!("tags.{tag}"): {"$exists": true}})
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

    /// 获取泰拉记事社最新一个小章节漫画
    #[instrument(skip(self), ret)]
    pub async fn get_newest_terra_comic_episode(
        &'db self,
    ) -> OperateResult<Option<CookieSimpleInfo>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<CookieSimpleInfo> =
            &collection.with_mapping();
        let res = collection
            .doing(|collection| {
                collection.find_one(
                    doc! {"meta.item.comic": {"$exists":true}},
                    FindOneOptions::builder()
                        .projection(doc! {
                            "meta": 1,
                            "text": 1,
                            "images": 1,
                        })
                        .sort(doc! {"_id": -1})
                        .build(),
                )
            })
            .await?;
        Ok(res)
    }
}
