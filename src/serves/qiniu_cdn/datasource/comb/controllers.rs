use axum::{extract::{rejection::PathRejection, Path}, Json};
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic, view::CombIdToCookieIdRep,
};
use http::StatusCode;
use persistence::redis::RedisConnect;
use tracing::instrument;

use crate::router::QiniuCdnDatasourceCombFrontend;

impl QiniuCdnDatasourceCombFrontend {
    #[instrument(ret, skip(redis_client))]
    pub async fn get_newest_cookie(
        redis_client: RedisConnect,
        comb_id_path: Result<Path<String>, PathRejection>,
    ) -> (StatusCode, Json<Option<CombIdToCookieIdRep>>) {
        match comb_id_path {
            Ok(Path(comb_id)) => {
                match CeobeCookieLogic::newest_comb_info(redis_client, comb_id).await {
                    Ok(combid_res) => (StatusCode::OK, Some(combid_res).into()),
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, None.into()),
                }
            }
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, None.into()),
        }
    }
}
