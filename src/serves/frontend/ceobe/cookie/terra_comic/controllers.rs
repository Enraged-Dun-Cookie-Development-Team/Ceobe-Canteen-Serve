use axum_macros::debug_handler;
use ceobe_cookie_logic::{view::{TerraCidReq, TerraComicListResp}, impletements::CeobeCookieLogic};
use mongo_migration::{mongo_connection::MongoDatabaseOperate, mongo_models::ceobe::cookie::analyze::models::TerraComicEpisodeInfo};
use resp_result::{MapReject, rtry};
use tracing::instrument;
use axum::extract::Query;

use crate::router::CookieTerraComicFrontend;

use super::error::{CeobeCookieRResult, CeobeCookieTerraComicError};

impl CookieTerraComicFrontend {
    #[instrument(ret, skip(mongo))]
    pub async fn comic_list(
        mongo: MongoDatabaseOperate,
    ) -> CeobeCookieRResult<Vec<TerraComicListResp>> {
        Ok(rtry!(
            CeobeCookieLogic::comic_list(
                mongo
            )
            .await
        ))
        .into()
    }

    #[instrument(ret, skip(mongo))]
    pub async fn comic_episode_list(
        mongo: MongoDatabaseOperate,
        MapReject(TerraCidReq {comic}): MapReject<
            Query<TerraCidReq>,
            CeobeCookieTerraComicError,
        >,
    ) -> CeobeCookieRResult<Vec<TerraComicEpisodeInfo>> {
        Ok(rtry!(
            CeobeCookieLogic::comic_episode_list(
                mongo,
                comic
            )
            .await
        ))
        .into()
    }
}
