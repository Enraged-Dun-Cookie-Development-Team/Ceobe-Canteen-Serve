use std::str::FromStr;

use db_ops_prelude::{
    mongo_connection::{MongoDbCollectionTrait, CollectionGuard, MongoDbError},
    mongo_models::ceobe::cookie::temp_list::models::{TempListModel, SingleData, CookieId},
    mongodb::{
        bson::{doc, Document, oid::ObjectId},
        options::{FindOptions, FindOneOptions},
    }, futures::StreamExt,
};
use tracing::instrument;

use crate::temp_list::OperateError;

use super::{OperateResult, TempListOperate};

impl<'db, Conn> TempListOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, TempListModel>,
{
    /// 分页查询饼数据
    #[instrument(skip(self), ret)]
    pub async fn get_data_by_paginate(
        &'db self, first_id: String, datasources: Vec<i32>, page_number: i64
    ) -> OperateResult<Vec<String>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<SingleData> = &collection.with_mapping();
        let filter = doc! {"$and": [{"source_config_id": {"$in":datasources}}, {"_id": {"$lte": ObjectId::from_str(&first_id).map_err(|_| OperateError::CookieIdError(first_id))?}}]};
        let mut vec = collection
            .doing(|collection| {
                collection.find(
                    filter,
                    FindOptions::builder().projection(doc! {"data":1, "_id":0}).sort(doc! {"_id": -1}).limit(page_number).build(),
                )
            })
            .await?;
        let mut res = Vec::<String>::new();
        while let Some(v) = vec.next().await {
            res.push(v.map_err(|err| MongoDbError::from(err))?.data);
        }
        Ok(res)
    }

    /// 获取下一页的饼id
    #[instrument(skip(self), ret)]
    pub async fn get_next_page_cookie_id(
        &'db self, first_id: String, datasources: Vec<i32>, page_number: u64
    ) -> OperateResult<Option<String>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<CookieId> = &collection.with_mapping();
        let filter = doc! {"$and": [{"source_config_id": {"$in":datasources}}, {"_id": {"$lte": ObjectId::from_str(&first_id).map_err(|_| OperateError::CookieIdError(first_id))?}}]};
        let cookie_id = collection
            .doing(|collection| {
                collection.find_one(
                    filter,
                    FindOneOptions::builder().projection(doc! {"_id":1}).sort(doc! {"_id": -1}).skip(page_number).build(),
                )
            })
            .await?;

        let res = match cookie_id {
            Some(id) => Some(id._id.to_string()),
            None => None,
        };

        Ok(res)
    }

    /// 获取数据源第一个饼id
    #[instrument(skip(self), ret)]
    pub async fn get_first_cookie_id(
        &'db self, datasources: Vec<i32>
    ) -> OperateResult<Option<String>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<CookieId> = &collection.with_mapping();
        let filter = doc! {"source_config_id": {"$in":datasources}};
        let cookie_id = collection
            .doing(|collection| {
                collection.find_one(
                    filter,
                    FindOneOptions::builder().projection(doc! {"_id":1}).sort(doc! {"_id": -1}).build(),
                )
            })
            .await?;

        let res = match cookie_id {
            Some(id) => Some(id._id.to_string()),
            None => None,
        };

        Ok(res)
    }
}
