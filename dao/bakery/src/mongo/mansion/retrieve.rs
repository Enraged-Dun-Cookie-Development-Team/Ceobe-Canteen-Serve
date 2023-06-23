use std::{iter::Iterator, ops::Add};

use db_ops_prelude::{
    chrono::{
        self,
        format::{DelayedFormat, StrftimeItems},
        Duration, Local,
    },
    futures::StreamExt,
    get_connect::GetDatabaseCollection,
    mongo_connection::{
        CollectionGuard, MongoDbCollectionTrait, MongoDbError,
    },
    mongo_models::bakery::mansion::preludes::{
        MansionId, Mid, ModelMansion, ModifyAt, RecentPredict,
    },
    mongodb::{
        bson::{self, doc, DateTime, Document},
        options::FindOptions,
    },
    tap::Tap,
};
use tracing::{info, instrument};

use super::{MansionOperate, OperateError, OperateResult};

impl<'db, Db> MansionOperate<'db, Db>
where
    Db: GetDatabaseCollection<ModelMansion> + 'db,
{
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
            .collect::<Vec<_>>()
            .tap(|list| info!(mansionList.ids = ?list)))
    }

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
}
impl<'db, Db> MansionOperate<'db, Db>
where
    Db: MongoDbCollectionTrait<'db, ModelMansion>,
{
    #[instrument(skip(self), ret)]
    /// 获取单一大厦信息
    /// params：mid 大厦id
    pub async fn get_mansion_by_id(
        &'db self, mid: &MansionId,
    ) -> OperateResult<ModelMansion> {
        info!(findMansion.id = %mid);

        let collection = self.get_collection()?;
        collection
            .doing(|collection| {
                collection.find_one(mid.into_id_filter(), None)
            })
            .await?
            .ok_or(OperateError::MansionNotFound)
    }

    #[instrument(skip_all)]
    /// 无条件获取大厦id列表
    pub async fn get_all_mansion_id_list(
        &'db self,
    ) -> OperateResult<Vec<String>> {
        let collection = self.get_collection()?;
        Self::get_mansion_id_list_by_filter(None, &collection.with_mapping())
            .await
    }

    #[instrument(skip_all)]
    /// 根据时间获取以来的大厦id列表
    /// params： time 往前多少时间
    pub async fn get_mansion_id_list_by_time(
        &'db self, time: Duration,
    ) -> OperateResult<Vec<String>> {
        info!(
            mansionList.recentTime = format!(
                "{}days {} h {} m {} s",
                time.num_days(),
                time.num_hours(),
                time.num_minutes(),
                time.num_seconds()
            )
        );

        let collection = self.get_collection()?;

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

    /// 获取最近一天的预测（明天后最近的预测）-如果同时多个大厦符合，
    /// 获取新大厦的
    pub async fn get_recent_predict(
        &'db self,
    ) -> OperateResult<Option<RecentPredict>> {
        const FMT: &str = "%Y-%m-%d";
        let now: chrono::prelude::DateTime<Local> =
            Local::now().add(Duration::days(1));
        let dft: DelayedFormat<StrftimeItems> = now.format(FMT);
        let str_now: String = dft.to_string();

        let collection = self.get_collection()?;
        let collection: &CollectionGuard<RecentPredict> =
            &collection.with_mapping();
        let mut pipeline = Vec::<Document>::new();

        let unwind = doc! {
            "$unwind": "$daily"
        };
        pipeline.push(unwind);

        let match_op = doc! {
            "$match": {
                "daily.datetime": {
                    "$gte": str_now
                }
            }
        };
        pipeline.push(match_op);

        let project = doc! {
            "$project": {
                "_id": 0,
                "id": "$id",
                "description":"$description",
                "daily": "$daily"
            }
        };
        pipeline.push(project);

        let sort = doc! {
            "$sort": {
                "daily.datetime": 1, "id": -1
            }
        };
        pipeline.push(sort);

        let limit = doc! {
            "$limit": 1
        };
        pipeline.push(limit);

        let mut vec = collection
            .doing(|collection| collection.aggregate(pipeline, None))
            .await?;

        let res = vec.next().await;

        Ok(match res {
            Some(predict) => Some(bson::from_document::<RecentPredict>(predict.map_err(MongoDbError::from)?)?),
            None => None,
        })
    }

    /// 获取最近一天的结果（今天前最近的预测）-如果同时多个大厦符合，
    /// 获取新大厦的
    #[instrument(skip(self), ret)]
    pub async fn get_recent_result(
        &'db self,
    ) -> OperateResult<Option<RecentPredict>> {
        const FMT: &str = "%Y-%m-%d";
        let now: chrono::prelude::DateTime<Local> = Local::now();
        let dft: DelayedFormat<StrftimeItems> = now.format(FMT);
        let str_now: String = dft.to_string();

        let collection = self.get_collection()?;
        let collection: &CollectionGuard<RecentPredict> =
            &collection.with_mapping();
        let mut pipeline = Vec::<Document>::new();

        let unwind = doc! {
            "$unwind": "$daily"
        };
        pipeline.push(unwind);

        let match_op = doc! {
            "$match": {
                "daily.datetime": {
                    "$lte": str_now
                }
            }
        };
        pipeline.push(match_op);

        let project = doc! {
            "$project": {
                "_id": 0,
                "id": "$id",
                "description":"$description",
                "daily": "$daily"
            }
        };
        pipeline.push(project);

        let sort = doc! {
            "$sort": {
                "daily.datetime": -1, "id": -1
            }
        };
        pipeline.push(sort);

        let limit = doc! {
            "$limit": 1
        };
        pipeline.push(limit);

        let mut vec = collection
            .doing(|collection| collection.aggregate(pipeline, None))
            .await?;
        let res = vec.next().await;

        Ok(match res {
            Some(predict) => Some(bson::from_document::<RecentPredict>(predict.map_err(MongoDbError::from)?)?),
            None => None,
        })
    }
}
