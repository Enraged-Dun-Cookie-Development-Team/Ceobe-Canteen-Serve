use checker::CheckExtract;
use modify_cache::CacheMode;
use mongo_migration::{
    mongo_connection::MongoConnect,
    mongo_models::bakery::mansion::operate::MansionDataMongoOperate,
};
use resp_result::{resp_try, FlagWrap};

use super::{
    models::MidCheckerPretreatment, view::MansionIds, FlagMansionRResult,
};
use crate::{
    router::BakeryMansionFrontend,
    serves::frontend::bakery_mansion::view::ViewMansionWithTime,
};

impl BakeryMansionFrontend {
    pub async fn get_mansion_with_time(
        db: MongoConnect, CheckExtract(mid, _): MidCheckerPretreatment,
        mut modify: modify_cache::CheckModify,
    ) -> FlagMansionRResult<ViewMansionWithTime> {
        resp_try(async {
            let ctrl = modify.cache_headers.get_control();
            ctrl.set_ty(CacheMode::NoCache);

            let (data, extra) = modify.check_modify(
                MansionDataMongoOperate::get_mansion_by_id(&db, &mid.id)
                    .await?,
            )?;

            Ok(FlagWrap::new(data.map(Into::into), extra))
        })
        .await
    }

    pub async fn get_all_id(
        db: MongoConnect, mut modify: modify_cache::CheckModify,
    ) -> FlagMansionRResult<Vec<String>> {
        resp_try(async {
            let ctrl = modify.cache_headers.get_control();
            ctrl.set_ty(CacheMode::NoCache);

            let (data, extra) = modify.check_modify(MansionIds(
                MansionDataMongoOperate::get_all_mansion_id_list(&db).await?,
            ))?;
            Ok(FlagWrap::new(MansionIds::into_inner(data), extra))
        })
        .await
    }
}
