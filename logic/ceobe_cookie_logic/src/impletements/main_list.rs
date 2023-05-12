use std::{collections::HashMap, sync::Arc};

use bitmap_convert::{
    base70::BitmapBase70Conv, vec_usize::BitmapVecUsizeConv,
};
use bitmaps::Bitmap;
use ceobe_cookie::{ToCeobe, ToCookie};
use db_ops_prelude::{
    get_connect::GetMutDatabaseConnect,
    mongo_connection::MongoDatabaseOperate,
    mongo_models::ceobe::cookie::analyze::models::{meta::Meta, CookieInfo},
    sql_models::fetcher::datasource_config::models::model_datasource_config::DatasourceBasicInfo,
    SqlDatabaseOperate,
};
use fetcher::{
    datasource_combination::ToDatasourceCombination,
    datasource_config::{
        OperateError as DatasourceOperateError, ToDatasource,
    },
    ToFetcher,
};
use redis::AsyncCommands;
use redis_connection::RedisConnect;
use redis_global::redis_key::cookie_list::CookieListKey;
use tokio::task::{self, JoinHandle};

use super::CeobeCookieLogic;
use crate::{
    error::{LogicError, LogicResult},
    view::{CookieListReq, CookieListResp, DefaultCookie, SingleCookie},
};

impl CeobeCookieLogic {
    pub async fn cookie_list(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        mut redis_client: RedisConnect, cookie_info: CookieListReq,
    ) -> LogicResult<CookieListResp> {
        let redis = redis_client.mut_connect();
        if !redis
            .exists(format!(
                "{}{}",
                CookieListKey::NEW_UPDATE_COOKIE_ID,
                cookie_info.update_cookie_id
            ))
            .await?
        {
            return Err(LogicError::UpdateCookieCacheExpire(
                cookie_info.update_cookie_id.to_string(),
            ));
        }
        // 转换数据源组合id成数据源ids
        let datasource_bitmap: Bitmap<256> = BitmapBase70Conv::from_base_70(
            cookie_info.datasource_comb_id.clone(),
        )?;
        let datasource_indexes: Arc<[i32]> = datasource_bitmap
            .bitmap_to_usize()
            .into_iter()
            .map(|index| index as i32)
            .collect::<Vec<i32>>()
            .into_boxed_slice()
            .into();

        // 获取饼列表
        let cookie_list = task::spawn({
            let mongo = mongo.clone();
            let datasource_indexes = Arc::clone(&datasource_indexes);
            async move {
                mongo
                    .ceobe()
                    .cookie()
                    .analyze()
                    .get_data_by_paginate(
                        cookie_info.cookie_id,
                        &datasource_indexes,
                        10,
                    )
                    .await
            }
        });
        // 获取最新页饼id
        let next_cookie_id = task::spawn({
            let datasource_indexes = Arc::clone(&datasource_indexes);
            async move {
                mongo
                    .ceobe()
                    .cookie()
                    .analyze()
                    .get_next_page_cookie_id(
                        cookie_info.cookie_id,
                        &datasource_indexes,
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

        // 更新该数据源组合活跃时间
        db.fetcher()
            .datasource_combination()
            .update_access_time(&cookie_info.datasource_comb_id)
            .await?;

        Ok(CookieListResp {
            cookies,
            next_page_id: next_cookie_id.map(|id| id.to_string()),
        })
    }
}
