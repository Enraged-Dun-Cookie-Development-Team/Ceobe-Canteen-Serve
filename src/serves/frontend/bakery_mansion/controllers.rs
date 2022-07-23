use axum_prehandle::PreHandling;
use mongo_migration::mongo_models::mansion_data::operate::MansionDataMongoOperate;

use super::{MansionRResult, MidCheckerPretreatment};
use crate::{
    router::BakeryMansionFrontend,
    serves::frontend::bakery_mansion::view::ViewMansionWithTime,
};

impl BakeryMansionFrontend {
    pub async fn get_mansion_with_time(
        PreHandling(mid): MidCheckerPretreatment,
    ) -> MansionRResult<ViewMansionWithTime> {
        Ok(MansionDataMongoOperate::get_mansion_by_id(&mid.id)
            .await?
            .into())
        .into()
    }

    pub async fn get_all_id() -> MansionRResult<Vec<String>> {
        Ok(MansionDataMongoOperate::get_all_mansion_id_list().await?).into()
    }
}
