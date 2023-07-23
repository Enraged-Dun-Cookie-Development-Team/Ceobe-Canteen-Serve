use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic, view::CookieNumberResp,
};
use resp_result::rtry;
use tracing::instrument;
use persistence::mongodb::MongoDatabaseOperate;

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
