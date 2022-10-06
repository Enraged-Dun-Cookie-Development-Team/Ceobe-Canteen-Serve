use axum_prehandle::PreHandling as ReqPretreatment;
use chrono::Duration;
use mongo_migration::{
    mongo_connection::MongoConnect,
    mongo_models::bakery::mansion::operate::MansionDataMongoOperate,
};

use super::{
    MansionBodyCheckerPretreatment, MansionRResult, MidCheckerPretreatment,
    OptionMidCheckerPretreatment,
};
use crate::{
    router::BakeryMansionBackend,
    serves::backend::bakery_mansion::view::ViewMansion,
};

impl BakeryMansionBackend {
    pub async fn save_mansion(
        db: MongoConnect, ReqPretreatment(mid): OptionMidCheckerPretreatment,
        ReqPretreatment(json): MansionBodyCheckerPretreatment,
    ) -> MansionRResult<()> {
        let mid = mid.id;
        let data = json;

        match mid {
            Some(mid) => {
                log::info!("MansionId已提供 => 更新模式");

                MansionDataMongoOperate::update_mansion(&db, mid, data)
                    .await?;
            }
            None => {
                log::info!("MansionId未提供 => 新建模式");
                MansionDataMongoOperate::create_mansion_data(&db, data)
                    .await?;
            }
        }
        Ok(()).into()
    }

    pub async fn get_mansion(
        db: MongoConnect, ReqPretreatment(mid): MidCheckerPretreatment,
    ) -> MansionRResult<ViewMansion> {
        let data =
            MansionDataMongoOperate::get_mansion_by_id(&db, &mid.id).await?;
        MansionRResult::ok(data.into())
    }

    pub async fn get_recent_id(
        db: MongoConnect,
    ) -> MansionRResult<Vec<String>> {
        let mansion_ids =
            MansionDataMongoOperate::get_mansion_id_list_by_time(
                &db,
                Duration::days(60),
            )
            .await?;

        Ok(mansion_ids).into()
    }

    pub async fn remove_mansion(
        db: MongoConnect, ReqPretreatment(mid): MidCheckerPretreatment,
    ) -> MansionRResult<()> {
        MansionDataMongoOperate::delete_mansion(&db, &mid.id).await?;
        Ok(()).into()
    }
}
