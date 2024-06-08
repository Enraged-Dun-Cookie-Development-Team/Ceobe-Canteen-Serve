use bakery_logic::{
    impletements::BakeryLogic, view::{MansionRecentPredictResp, MansionWithTimeResp},
};
use checker::CheckExtract;
use modify_cache::CacheMode;
use persistence::{
    bakery::{mansion::ToMansion, ToBakery},
    mongodb::MongoDatabaseOperate,
};
use resp_result::{resp_try, rtry, FlagWrap};
use tracing::instrument;

use super::{
    error::MidCheckerPretreatment
};
use crate::{
    router::CdnBakeryMansionFrontend,
};
use super::error::MansionRResult;

impl CdnBakeryMansionFrontend {
    #[instrument(skip(db))]
    pub async fn get_mansion_with_time(
        db: MongoDatabaseOperate, CheckExtract(mid): MidCheckerPretreatment,
    ) -> MansionRResult<MansionWithTimeResp> {
        resp_try(async {
            Ok(BakeryLogic::get_mansion(db, mid).await?.into())
        })
        .await
    }

    #[instrument(skip(db))]
    pub async fn get_all_id(
        db: MongoDatabaseOperate
    ) -> MansionRResult<Vec<String>> {
        resp_try(async {
            Ok(BakeryLogic::get_all_mansion_id(db).await?)
        })
        .await
    }

    #[instrument(skip(mongo))]
    pub async fn recent_mansion_predict(
        mongo: MongoDatabaseOperate,
    ) -> MansionRResult<Option<MansionRecentPredictResp>> {
        Ok(rtry!(BakeryLogic::recent_mansion_predict(mongo).await)).into()
    }
}
