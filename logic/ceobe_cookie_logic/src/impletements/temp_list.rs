use bitmap_convert::{
    base70::BitmapBase70Conv, vec_usize::BitmapVecUsizeConv,
};
use bitmaps::Bitmap;
use futures::future;
use persistence::{
    ceobe_cookie::{ToCeobe, ToCookie},
    fetcher::{datasource_combination::ToDatasourceCombination, ToFetcher},
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
};

use crate::{
    error::LogicResult,
    impletements::CeobeCookieLogic,
    view::{CookieListReq, CookieTempListResp},
};

impl CeobeCookieLogic {
    pub async fn get_temp_cookies_by_pagenation(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        cookie_info: CookieListReq,
    ) -> LogicResult<CookieTempListResp> {
        let datasource_map: Bitmap<256> = BitmapBase70Conv::from_base_70(
            cookie_info.datasource_comb_id.clone(),
        )?;
        let datasource_vec = datasource_map
            .bitmap_to_usize()
            .into_iter()
            .map(|index| index as i32)
            .collect::<Vec<i32>>();

        let (cookie_list, next_cookie_id) = future::join(
            mongo.ceobe().cookie().temp_list().get_data_by_paginate(
                cookie_info.cookie_id,
                datasource_vec.clone(),
                10,
            ),
            mongo.ceobe().cookie().temp_list().get_next_page_cookie_id(
                cookie_info.cookie_id,
                datasource_vec,
                10,
            ),
        )
        .await;

        let cookie_list = cookie_list?
            .into_iter()
            .map(|str| serde_json::from_str(&str).unwrap())
            .collect();

        // 更新该数据源组合活跃时间
        db.fetcher()
            .datasource_combination()
            .update_access_time(&cookie_info.datasource_comb_id)
            .await?;

        Ok(CookieTempListResp {
            cookies: cookie_list,
            next_page_id: next_cookie_id?.map(|id| id.to_string()),
        })
    }
}
