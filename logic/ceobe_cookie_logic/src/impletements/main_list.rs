use std::collections::HashMap;

use bitmap_convert::{
    base70::BitmapBase70Conv, vec_usize::BitmapVecUsizeConv,
};
use bitmaps::Bitmap;
use ceobe_cookie::{ToCeobe, ToCookie};
use db_ops_prelude::{
    mongo_connection::MongoDatabaseOperate,
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
use tokio::task::{self, JoinHandle};

use super::CeobeCookieLogic;
use crate::{
    error::LogicResult,
    view::{CookieListReq, CookieListResp, DefaultCookie, SingleCookie},
};

impl CeobeCookieLogic {
    pub async fn cookie_list(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        cookie_info: CookieListReq,
    ) -> LogicResult<CookieListResp> {
        // 转换数据源组合id成数据源ids
        let datasource_map: Bitmap<256> = BitmapBase70Conv::from_base_70(
            cookie_info.datasource_comb_id.clone(),
        )?;
        let datasource_vec = datasource_map
            .bitmap_to_usize()
            .into_iter()
            .map(|index| index as i32)
            .collect::<Vec<i32>>();

        // 获取饼列表
        let mongo_copy = mongo.clone();
        let datasource_vec_copy = datasource_vec.clone();
        let cookie_list = task::spawn(async move {
            mongo_copy
                .ceobe()
                .cookie()
                .analyze()
                .get_data_by_paginate(
                    cookie_info.cookie_id,
                    datasource_vec_copy,
                    10,
                )
                .await
        });
        // 获取最新页饼id
        let datasource_vec_copy = datasource_vec.clone();
        let next_cookie_id = task::spawn(async move {
            mongo
                .ceobe()
                .cookie()
                .analyze()
                .get_next_page_cookie_id(
                    cookie_info.cookie_id,
                    datasource_vec_copy,
                    10,
                )
                .await
        });
        // 获取数据源基本信息
        let db_copy = db.clone();
        let datasource_info: JoinHandle<Result<_, DatasourceOperateError>> =
            task::spawn(async move {
                let basic_info = db_copy
                    .fetcher()
                    .datasource()
                    .find_basic_info_by_ids(datasource_vec)
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
        let mut cookie_list_resq = Vec::<SingleCookie>::new();
        for cookie in cookie_list {
            cookie_list_resq.push(
                SingleCookie::builder()
                    .datasource(
                        datasource_info[&cookie.source_config_id]
                            .nickname
                            .clone(),
                    )
                    .icon(
                        datasource_info[&cookie.source_config_id]
                            .avatar
                            .clone(),
                    )
                    .jump_url(cookie.meta.item.url)
                    .timestamp(cookie.meta.timestamp)
                    .default_cookie(
                        DefaultCookie::builder()
                            .text(cookie.text)
                            .images(cookie.images)
                            .compress_images(cookie.compress_images)
                            .build(),
                    )
                    .build(),
            );
        }

        // 更新该数据源组合活跃时间
        db.fetcher()
            .datasource_combination()
            .update_access_time(&cookie_info.datasource_comb_id)
            .await?;

        Ok(CookieListResp {
            cookies: cookie_list_resq,
            next_page_id: next_cookie_id.map(|id| id.to_string()),
        })
    }
}
