use axum_resp_result::rtry;
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic, view::CookieNumberResp,
};
use persistence::mongodb::MongoDatabaseOperate;
use tracing::instrument;

use super::error::CeobeCookieRResult;
use crate::router::CookieInfoFrontend;

impl CookieInfoFrontend {
    #[instrument(ret, skip(mongo))]
    pub async fn cookie_count(
        mongo: MongoDatabaseOperate,
    ) -> CeobeCookieRResult<CookieNumberResp> {
        Ok(rtry!(CeobeCookieLogic::cookie_number(mongo).await)).into()
    }
}
