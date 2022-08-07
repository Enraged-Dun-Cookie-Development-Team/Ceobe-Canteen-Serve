use std::time::Duration;

use axum_prehandle::PreHandling;
use mongo_migration::mongo_models::bakery::mansion::operate::MansionDataMongoOperate;
use resp_result::RespResult;

use super::{view::MansionIds, FLagMansionRResult, MidCheckerPretreatment};
use crate::{
    router::BakeryMansionFrontend,
    serves::frontend::bakery_mansion::view::ViewMansionWithTime,
};

impl BakeryMansionFrontend {
    pub async fn get_mansion_with_time(
        PreHandling(mid): MidCheckerPretreatment,
        modify: modify_cache::CheckModify,
    ) -> FLagMansionRResult<ViewMansionWithTime> {
        let (data, extra) = modify.is_modify(
            MansionDataMongoOperate::get_mansion_by_id(&mid.id).await?,
        )?;

        RespResult::ok(data.map(Into::into)).with_flags(extra)
    }

    pub async fn get_all_id(
        mut modify: modify_cache::CheckModify,
    ) -> FLagMansionRResult<Vec<String>> {
        let ctrl = modify.cache_info.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60 * 4));
        let (data, extra) = modify.is_modify(MansionIds(
            MansionDataMongoOperate::get_all_mansion_id_list().await?,
        ))?;

        RespResult::ok(data.map(|v| v.0)).with_flags(extra)
    }
}
