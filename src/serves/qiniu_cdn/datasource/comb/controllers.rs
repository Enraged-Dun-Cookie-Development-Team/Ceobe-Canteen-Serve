use axum::{debug_handler, extract::Path, Json};
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic,
    view::CombIdToCookieIdReq,
};
use http::StatusCode;
use persistence::redis::RedisConnect;
use resp_result::MapReject;
use tracing::instrument;

use super::error::QiniuCdnDatasourceCombError;
use crate::router::QiniuCdnDatasourceCombFrontend;

impl QiniuCdnDatasourceCombFrontend {
    #[instrument(ret, skip(redis_client))]
    // #[debug_handler]
    pub async fn get_newest_cookie(
        redis_client: RedisConnect,
        MapReject(comb_id): MapReject<Path<String>, QiniuCdnDatasourceCombError>,
    ) -> (StatusCode, Json<Option<CombIdToCookieIdReq>>) {
        match CeobeCookieLogic::newest_comb_info(redis_client, comb_id).await
        {
            Ok(combid_res) => (StatusCode::OK, Some(combid_res).into()),
            Err(_) => (StatusCode::from_u16(500).unwrap(), None.into()),
        }
    }
}
