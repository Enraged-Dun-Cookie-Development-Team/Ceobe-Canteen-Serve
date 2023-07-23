use axum::extract::Query;
use ceobe_cookie_logic::{
    impletements::CeobeCookieLogic,
    view::{TerraCidReq, TerraComicListResp, TerraEntryResp},
};
use persistence::{
    ceobe_cookie::models::analyze::models::TerraComicEpisodeInfo,
    mongodb::MongoDatabaseOperate,
};
use resp_result::{rtry, MapReject};
use tracing::instrument;

use super::error::{CeobeCookieRResult, CeobeCookieTerraComicError};
use crate::router::CookieTerraComicFrontend;

impl CookieTerraComicFrontend {
    #[instrument(ret, skip(mongo))]
    pub async fn comic_list(
        mongo: MongoDatabaseOperate,
    ) -> CeobeCookieRResult<Vec<TerraComicListResp>> {
        Ok(rtry!(CeobeCookieLogic::comic_list(mongo).await)).into()
    }

    #[instrument(ret, skip(mongo))]
    pub async fn comic_episode_list(
        mongo: MongoDatabaseOperate,
        MapReject(TerraCidReq { comic }): MapReject<
            Query<TerraCidReq>,
            CeobeCookieTerraComicError,
        >,
    ) -> CeobeCookieRResult<Vec<TerraComicEpisodeInfo>> {
        Ok(rtry!(
            CeobeCookieLogic::comic_episode_list(mongo, comic).await
        ))
        .into()
    }

    #[instrument(ret, skip(mongo))]
    pub async fn newest_episode(
        mongo: MongoDatabaseOperate,
    ) -> CeobeCookieRResult<Option<TerraEntryResp>> {
        Ok(rtry!(CeobeCookieLogic::newest_episode(mongo).await)).into()
    }
}
