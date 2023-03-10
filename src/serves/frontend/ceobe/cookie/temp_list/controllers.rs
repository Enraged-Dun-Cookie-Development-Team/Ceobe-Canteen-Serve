use axum::extract::Query;
use ceobe_cookie_logic::{implement::CeobeCookieLogic, view::{CookieListResp, CookieListReq}};
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use resp_result::{rtry, MapReject};
use tracing::instrument;

use crate::router::CookieTempFrontend;

use super::error::{CeobeCookieRResult, CeobeCookieTempListError};

impl CookieTempFrontend {
    #[instrument(ret, skip(mongo))]
    pub async fn cookie_list(
        mongo: MongoDatabaseOperate,
        MapReject(cookie_req_info): MapReject<Query<CookieListReq>, CeobeCookieTempListError>,
    ) -> CeobeCookieRResult<CookieListResp> {
        Ok(rtry!(CeobeCookieLogic::get_temp_cookies_by_pagenation(mongo, cookie_req_info).await)
        ).into()
    }
}