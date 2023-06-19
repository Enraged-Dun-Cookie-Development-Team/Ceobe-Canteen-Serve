use axum::extract::Query;
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic,
    view::{TerraCidReq, TerraComicListResp, CookieNumberResp},
};
use mongo_migration::{
    mongo_connection::MongoDatabaseOperate,
    mongo_models::ceobe::cookie::analyze::models::TerraComicEpisodeInfo,
};
use resp_result::{rtry, MapReject};
use tracing::instrument;

use super::error::{CeobeCookieRResult};
use crate::router::CookieInfoFrontend;

impl CookieInfoFrontend {
    #[instrument(ret, skip(mongo))]
    pub async fn cookie_count(
        mongo: MongoDatabaseOperate,
    ) -> CeobeCookieRResult<CookieNumberResp> {
        Ok(rtry!(CeobeCookieLogic::cookie_number(mongo).await)).into()
    }
}
