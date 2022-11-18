use checker::CheckExtract;
use chrono::Duration;
use mongo_migration::{
    mongo_connection::MongoConnect,
    mongo_models::bakery::mansion::operate::MansionDataMongoOperate,
};
use resp_result::resp_try;
use tracing::{instrument, log};

use super::{
    models::{
        MansionBodyCheckerPretreatment, MidCheckerPretreatment,
        OptionMidCheckerPretreatment,
    },
    MansionRResult,
};
use crate::{
    router::BakeryMansionBackend,
    serves::backend::bakery_mansion::view::ViewMansion,
};

impl BakeryMansionBackend {
    #[instrument(ret)]
    pub async fn save_mansion(
        db: MongoConnect,
        CheckExtract(mid, ..): OptionMidCheckerPretreatment,
        CheckExtract(json, ..): MansionBodyCheckerPretreatment,
    ) -> MansionRResult<()> {
        resp_try(async move {
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
            Ok(())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn get_mansion(
        db: MongoConnect, CheckExtract(mid, ..): MidCheckerPretreatment,
    ) -> MansionRResult<ViewMansion> {
        resp_try(async {
            Ok(MansionDataMongoOperate::get_mansion_by_id(&db, &mid.id)
                .await?
                .into())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn get_recent_id(
        db: MongoConnect,
    ) -> MansionRResult<Vec<String>> {
        resp_try(async {
            Ok(MansionDataMongoOperate::get_mansion_id_list_by_time(
                &db,
                Duration::days(60),
            )
            .await?)
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn remove_mansion(
        db: MongoConnect, CheckExtract(mid, ..): MidCheckerPretreatment,
    ) -> MansionRResult<()> {
        resp_try(async {
            MansionDataMongoOperate::delete_mansion(&db, &mid.id).await?;
            Ok(())
        })
        .await
    }
}
