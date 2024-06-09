use bakery_logic::{
    impletements::BakeryLogic, view::MansionRecentPredictResp,
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
    models::MidCheckerPretreatment, view::MansionIds, FlagMansionRResult,
    MansionRResult,
};
use crate::{
    router::BakeryMansionFrontend,
    serves::frontend::bakery_mansion::view::ViewMansionWithTime,
};

impl BakeryMansionFrontend {
    #[instrument(skip(db, modify))]
    pub async fn get_mansion_with_time(
        db: MongoDatabaseOperate, CheckExtract(mid): MidCheckerPretreatment,
        mut modify: modify_cache::CheckModify,
    ) -> FlagMansionRResult<ViewMansionWithTime> {
        resp_try(async {
            let ctrl = modify.cache_headers.get_control();
            ctrl.set_ty(CacheMode::NoCache);

            let (data, extra) = modify.check_modify(
                db.bakery().mansion().get_mansion_by_id(&mid.id).await?,
            )?;

            Ok(FlagWrap::new(data.map(Into::into), extra))
        })
        .await
    }

    #[instrument(skip(db, modify))]
    pub async fn get_all_id(
        db: MongoDatabaseOperate, mut modify: modify_cache::CheckModify,
    ) -> FlagMansionRResult<Vec<String>> {
        resp_try(async {
            let ctrl = modify.cache_headers.get_control();
            ctrl.set_ty(CacheMode::NoCache);

            let (data, extra) = modify.check_modify(MansionIds(
                db.bakery().mansion().get_all_mansion_id_list().await?,
            ))?;
            Ok(FlagWrap::new(MansionIds::into_inner(data), extra))
        })
        .await
    }

    #[instrument(skip(mongo))]
    pub async fn recent_mansion_predict(
        mongo: MongoDatabaseOperate,
    ) -> MansionRResult<Option<MansionRecentPredictResp>> {
        resp_try(async { Ok(BakeryLogic::recent_mansion_predict(mongo).await?) }).await
    }
}
