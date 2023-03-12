use abstract_database::ceobe::ToCeobe;
use bitmap_convert::{base70::BitmapBase70Conv, vec_i32::BitmapVecI32Conv};
use bitmaps::Bitmap;
use ceobe_cookie::ToCookie;
use futures::future;
use mongo_migration::mongo_connection::MongoDatabaseOperate;

use crate::{
    error::LogicResult,
    impletements::CeobeCookieLogic,
    view::{CookieListReq, CookieListResp},
};

impl CeobeCookieLogic {
    pub async fn get_temp_cookies_by_pagenation(
        db: MongoDatabaseOperate, cookie_info: CookieListReq,
    ) -> LogicResult<CookieListResp> {
        let datasource_map: Bitmap<256> =
            BitmapBase70Conv::base_70_to(cookie_info.datasource_comb_id)?;
        let datasource_vec = datasource_map
            .bitmap_to_i32()
            .into_iter()
            .map(|id| id)
            .collect::<Vec<i32>>();

        let (cookie_list, next_cookie_id) = future::join(
            db.ceobe().cookie().temp_list().get_data_by_paginate(
                cookie_info.cookie_id.clone(),
                datasource_vec.clone(),
                10,
            ),
            db.ceobe().cookie().temp_list().get_next_page_cookie_id(
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

        Ok(CookieListResp {
            cookies: cookie_list,
            next_page_id: next_cookie_id?,
        })
    }
}
