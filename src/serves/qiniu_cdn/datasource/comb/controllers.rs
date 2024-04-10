use axum::{extract::Path, Json};
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic, view::CombIdToCookieIdRep,
};
use http::StatusCode;
use persistence::redis::RedisConnect;
use resp_result::MapReject;
use tracing::instrument;

use crate::router::QiniuCdnDatasourceCombFrontend;

impl QiniuCdnDatasourceCombFrontend {
    #[instrument(ret, skip(redis_client))]
    pub async fn get_newest_cookie(
        redis_client: RedisConnect,
        MapReject(comb_id): Result<Path<String>,PathRejection>,
    ) -> (StatusCode, Json<Option<CombIdToCookieIdRep>>) {
        match CeobeCookieLogic::newest_comb_info(redis_client, comb_id).await
        {
            Ok(combid_res) => (StatusCode::OK, Some(combid_res).into()),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, None.into()),
        }
    }
}
