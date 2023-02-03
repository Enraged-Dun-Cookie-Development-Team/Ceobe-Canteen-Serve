use checker::CheckExtract;
use modify_cache::CacheMode;
use mongo_migration::{
    mongo_connection::MongoDatabaseOperate,
    mongo_models::bakery::mansion::operate::ToMansionOperate,
};
use resp_result::{resp_try, FlagWrap};
use tracing::instrument;

use super::{
    models::MidCheckerPretreatment, view::MansionIds, FlagMansionRResult,
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
                db.mansion().get_mansion_by_id(&mid.id).await?,
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
                db.mansion().get_all_mansion_id_list().await?,
            ))?;
            Ok(FlagWrap::new(MansionIds::into_inner(data), extra))
        })
        .await
    }
}
