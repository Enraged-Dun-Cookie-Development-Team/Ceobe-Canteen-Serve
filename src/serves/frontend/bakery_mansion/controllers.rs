use axum_prehandle::PreHandling;
use modify_cache::CacheMode;
use mongo_migration::mongo_models::bakery::mansion::operate::MansionDataMongoOperate;
use resp_result::RespResult;

use super::{view::MansionIds, FlagMansionRResult, MidCheckerPretreatment};
use crate::{
    router::BakeryMansionFrontend,
    serves::frontend::bakery_mansion::view::ViewMansionWithTime,
};

impl BakeryMansionFrontend {
    pub async fn get_mansion_with_time(
        PreHandling(mid): MidCheckerPretreatment,
        mut modify: modify_cache::CheckModify,
    ) -> FlagMansionRResult<ViewMansionWithTime> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_ty(CacheMode::NoCache);

        let (data, extra) = modify.check_modify(
            MansionDataMongoOperate::get_mansion_by_id(&mid.id).await?,
        )?;

        RespResult::ok(data.map(Into::into)).with_flags(extra)
    }

    pub async fn get_all_id(
        mut modify: modify_cache::CheckModify,
    ) -> FlagMansionRResult<Vec<String>> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_ty(CacheMode::NoCache);

        let (data, extra) = modify.check_modify(MansionIds(
            MansionDataMongoOperate::get_all_mansion_id_list().await?,
        ))?;

        RespResult::ok(data)
            .map(MansionIds::into_inner)
            .with_flags(extra)
    }
}
