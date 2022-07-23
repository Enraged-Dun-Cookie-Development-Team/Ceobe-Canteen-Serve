use chrono::Duration;
use mongo_migration::mongo_models::mansion_data::operate::MansionDataMongoOperate;

use super::{
    MansionAuthentication, MansionBodyCheckerPretreatment, MansionRResult,
    MidCheckerPretreatment, OptionMidCheckerPretreatment,
};
use crate::{
    router::BakeryMansionBackend,
    serves::backend::bakery_mansion::view::ViewMansion,
};
use axum_prehandle::PreHandling as ReqPretreatment;

impl BakeryMansionBackend {
    pub async fn save_mansion(
        _: MansionAuthentication,
        ReqPretreatment(mid): OptionMidCheckerPretreatment,
        ReqPretreatment(json): MansionBodyCheckerPretreatment,
    ) -> MansionRResult<()> {
        let mid = mid.id;
        let data = json;

        match mid {
            Some(mid) => {
                log::info!("MansionId已提供 => 更新模式");

                MansionDataMongoOperate::update_mansion(mid, data).await?;
            }
            None => {
                log::info!("MansionId未提供 => 新建模式");
                MansionDataMongoOperate::create_mansion_data(data).await?;
            }
        }
        Ok(()).into()
    }

    pub async fn get_mansion(
        _: MansionAuthentication,
        ReqPretreatment(mid): MidCheckerPretreatment,
    ) -> MansionRResult<ViewMansion> {
        let data =
            MansionDataMongoOperate::get_mansion_by_id(&mid.id).await?;
        MansionRResult::ok(data.into())
    }

    pub async fn get_recent_id(
        _: MansionAuthentication,
    ) -> MansionRResult<Vec<String>> {
        let mansion_ids =
            MansionDataMongoOperate::get_mansion_id_list_by_time(
                Duration::days(60),
            )
            .await?;

        Ok(mansion_ids).into()
    }

    pub async fn remove_mansion(
        _: MansionAuthentication,
        ReqPretreatment(mid): MidCheckerPretreatment,
    ) -> MansionRResult<()> {
        MansionDataMongoOperate::delete_mansion(&mid.id).await?;
        Ok(()).into()
    }
}
