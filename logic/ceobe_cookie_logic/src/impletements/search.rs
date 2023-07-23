use std::{collections::HashMap, str::FromStr};

use bitmap_convert::{
    base70::BitmapBase70Conv, vec_usize::BitmapVecUsizeConv,
};
use bitmaps::Bitmap;
use persistence::{
    prelude::mongodb::bson::oid::ObjectId,
    ceobe_cookie::models::analyze::models::{meta::Meta, CookieInfo},
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
    ceobe_cookie::{ToCeobe, ToCookie},
    fetcher::models::datasource_config::models::model_datasource_config::DatasourceBasicInfo,
    fetcher::{
        datasource_config::{
            OperateError as DatasourceOperateError, ToDatasource,
        },
        ToFetcher,
    }
};
use tokio::task::{self, JoinHandle};

use super::CeobeCookieLogic;
use crate::{
    error::{LogicError, LogicResult},
    view::{CookieListResp, DefaultCookie, SearchListReq, SingleCookie},
};

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

        // mongo关键词搜索
        let mut cookie_list = mongo
            .ceobe()
            .cookie()
            .analyze()
            .get_data_by_paginate_and_keyword(
                search_info.cookie_id,
                datasource_indexes.as_slice(),
                10,
                &search_info.search_word,
            )
            .await?;

        let next_cookie_id: JoinHandle<Result<Option<String>, LogicError>>;
        // 如果关键词搜索没有搜索到结构，采用sql全文搜索
        if cookie_list.is_empty() {
            // 获取饼列表
            let cookie_list_handle: JoinHandle<
                Result<Vec<CookieInfo>, LogicError>,
            > = task::spawn({
                let db = db.clone();
                let datasource_indexes = datasource_indexes.clone();
                let search_info = search_info.clone();
                async move {
                    let object_id_str_list = db
                        .ceobe()
                        .cookie()
                        .search_content()
                        .get_page_cookie_ids(
                            search_info
                                .cookie_id
                                .map(|item| item.to_string()),
                            &search_info.search_word,
                            &datasource_indexes,
                            10,
                        )
                        .await?;
                    let mut object_id_list = Vec::new();
                    for object_id in object_id_str_list.into_iter() {
                        object_id_list.push(ObjectId::from_str(&object_id)?);
                    }
                    let cookies_map = mongo
                        .ceobe()
                        .cookie()
                        .analyze()
                        .get_data_by_object_ids(&object_id_list)
                        .await?
                        .into_iter()
                        .map(|info| (info._id, info.cookie_info))
                        .collect::<HashMap<ObjectId, CookieInfo>>();
                    Ok(object_id_list
                        .into_iter()
                        .filter(|id| cookies_map.contains_key(id))
                        .map(|id| cookies_map.get(&id).unwrap().to_owned())
                        .collect())
                }
            });
            // 获取下一页页饼id
            next_cookie_id = task::spawn({
                let db = db.clone();
                let datasource_indexes = datasource_indexes.clone();
                async move {
                    db.ceobe()
                        .cookie()
                        .search_content()
                        .get_next_page_cookie_id(
                            search_info
                                .cookie_id
                                .map(|item| item.to_string()),
                            &search_info.search_word,
                            &datasource_indexes,
                            10,
                        )
                        .await
                        .map_err(|err| err.into())
                }
            });
            cookie_list = cookie_list_handle.await??;
        }
        else {
            next_cookie_id = task::spawn({
                let datasource_indexes = datasource_indexes.clone();
                async move {
                    mongo
                        .ceobe()
                        .cookie()
                        .analyze()
                        .get_next_page_cookie_id_by_keyword(
                            search_info.cookie_id,
                            datasource_indexes.as_slice(),
                            10,
                            &search_info.search_word,
                        )
                        .await
                        .map(|option_oid| {
                            option_oid.map(|oid| oid.to_string())
                        })
                        .map_err(|err| err.into())
                }
            });
        }

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
        let next_cookie_id = next_cookie_id.await??;
        let datasource_info = datasource_info.await??;

        // 拼接数据
        let cookies = cookie_list
            .into_iter()
            .map(
                |CookieInfo {
                     meta:
                         Meta {
                             item,
                             timestamp,
                             source,
                             ..
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
                        .source(source)
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
