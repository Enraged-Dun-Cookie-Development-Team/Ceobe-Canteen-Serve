use bakery_logic::{impletements::BakeryLogic, view::MansionResp};
use checker::CheckExtract;
use persistence::mongodb::MongoDatabaseOperate;
use axum_resp_result::resp_try;
use tencent_cloud_server::cloud_manager::TencentCloudManager;
use tracing::instrument;

use super::{
    models::{
        MansionBodyCheckerPretreatment, MidCheckerPretreatment,
        OptionMidCheckerPretreatment,
    },
    MansionRResult,
};
use crate::router::BakeryMansionBackend;

impl BakeryMansionBackend {
    #[instrument(skip(db, tc_cloud), ret)]
    pub async fn save_mansion(
        db: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        CheckExtract(mid, ..): OptionMidCheckerPretreatment,
        CheckExtract(json, ..): MansionBodyCheckerPretreatment,
    ) -> MansionRResult<()> {
        resp_try(async move {
            let mid = mid.id;
            let data = json;

            BakeryLogic::save_mansion(db, tc_cloud, mid, data).await?;
            Ok(())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn get_mansion(
        db: MongoDatabaseOperate,
        CheckExtract(mid, ..): MidCheckerPretreatment,
    ) -> MansionRResult<MansionResp> {
        resp_try(async {
            Ok(BakeryLogic::get_mansion(db, mid).await?.into())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn get_recent_id(
        db: MongoDatabaseOperate,
    ) -> MansionRResult<Vec<String>> {
        resp_try(async { Ok(BakeryLogic::get_recent_id_by_90(db).await?) })
            .await
    }

    #[instrument(ret, skip(db, tc_cloud))]
    pub async fn remove_mansion(
        db: MongoDatabaseOperate, tc_cloud: TencentCloudManager,
        CheckExtract(mid, ..): MidCheckerPretreatment,
    ) -> MansionRResult<()> {
        resp_try(async {
            BakeryLogic::remove_mansion(db, tc_cloud, mid).await?;
            Ok(())
        })
        .await
    }
}
