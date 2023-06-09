use std::{sync::Arc, collections::HashMap, str::FromStr};

use bitmap_convert::{base70::BitmapBase70Conv, vec_usize::BitmapVecUsizeConv};
use bitmaps::Bitmap;
use ceobe_cookie::{ToCeobe, ToCookie};
use db_ops_prelude::{SqlDatabaseOperate, mongo_connection::MongoDatabaseOperate, sql_models::fetcher::datasource_config::models::model_datasource_config::DatasourceBasicInfo, mongo_models::ceobe::cookie::analyze::models::{CookieInfo, meta::Meta, CookieInfoWithId}, mongodb::bson::oid::ObjectId};
use redis_global::redis_key::concat_key;
use tokio::task::{self, JoinHandle};
use fetcher::{
    datasource_config::{
        OperateError as DatasourceOperateError, ToDatasource,
    },
    ToFetcher,
};

use crate::{error::{LogicError, LogicResult}, view::{CookieListResp, SearchListReq, SingleCookie, DefaultCookie}};

use super::CeobeCookieLogic;


impl CeobeCookieLogic {
    pub async fn search_list(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        search_info: SearchListReq,
    ) -> LogicResult<CookieListResp> {
        
        // 转换数据源组合id成数据源ids
        let datasource_bitmap: Bitmap<256> = BitmapBase70Conv::from_base_70(
            search_info.datasource_comb_id.clone(),
        )?;
        let datasource_indexes: Vec<i32> = datasource_bitmap
            .bitmap_to_usize()
            .into_iter()
            .map(|index| index as i32)
            .collect::<Vec<i32>>();

        // 获取饼列表
        let cookie_list: JoinHandle<Result<Vec<CookieInfo>, LogicError>> = task::spawn({
            let db = db.clone();
            let datasource_indexes = datasource_indexes.clone();
            let search_info = search_info.clone();
            async move{
                let object_id_str_list = db
                        .ceobe()
                        .cookie()
                        .search_content()
                        .get_page_cookie_ids(
                            search_info.cookie_id.map(|item| item.to_string()),
                            &search_info.search_word,
                            datasource_indexes,
                            10,
                        )
                        .await?;
                let mut object_id_list = Vec::new::<>();
                for object_id in object_id_str_list.into_iter() {
                    object_id_list.push(ObjectId::from_str(&object_id)?);
                }
                let cookies_map = mongo
                        .ceobe()
                        .cookie()
                        .analyze()
                        .get_data_by_object_ids(object_id_list.clone())
                        .await?
                        .into_iter()
                        .map(|info| (info._id, info.cookie_info))
                        .collect::<HashMap<ObjectId, CookieInfo>>();
                Ok(object_id_list
                    .into_iter()
                    .filter(|id| cookies_map.contains_key(id))
                    .map(|id| cookies_map.get(&id).unwrap().to_owned())
                    .collect())
        }});
        // 获取下一页页饼id
        let next_cookie_id = task::spawn({
            let db = db.clone();
            let datasource_indexes = datasource_indexes.clone();
            async move {
                db
                    .ceobe()
                    .cookie()
                    .search_content()
                    .get_next_page_cookie_id(
                        search_info.cookie_id.map(|item| item.to_string()),
                        &search_info.search_word,
                        datasource_indexes,
                        10,
                    )
                    .await
            }
        });
        // 获取数据源基本信息
        let db_copy = db.clone();
        let datasource_info: JoinHandle<Result<_, DatasourceOperateError>> =
            task::spawn(async move {
                let basic_info = db_copy
                    .fetcher()
                    .datasource()
                    .find_basic_info_by_ids(&datasource_indexes)
                    .await?;
                Ok(basic_info
                    .into_iter()
                    .map(|info| (info.id, info))
                    .collect::<HashMap<i32, DatasourceBasicInfo>>())
            });
        let cookie_list = cookie_list.await??;
        let next_cookie_id = next_cookie_id.await??;
        let datasource_info = datasource_info.await??;

        // 拼接数据
        let cookies = cookie_list
            .into_iter()
            .map(
                |CookieInfo {
                     meta:
                         Meta {
                             item, timestamp, ..
                         },
                     source_config_id,
                     text,
                     images,
                     ..
                 }| {
                    let (nickname, avatar) =
                        if let Some(DatasourceBasicInfo {
                            nickname,
                            avatar,
                            ..
                        }) = datasource_info.get(&source_config_id)
                        {
                            (nickname.to_owned(), avatar.to_owned())
                        }
                        else {
                            unreachable!("cannot find match datasource")
                        };
                    SingleCookie::builder()
                        .datasource(nickname)
                        .icon(avatar)
                        .item(item)
                        .timestamp(timestamp)
                        .default_cookie(DefaultCookie { text, images })
                        .build()
                },
            )
            .collect::<Vec<_>>();

        Ok(CookieListResp {
            cookies,
            next_page_id: next_cookie_id,
        })
    }
}