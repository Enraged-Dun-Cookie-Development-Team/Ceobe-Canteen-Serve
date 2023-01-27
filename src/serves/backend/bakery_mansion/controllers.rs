use checker::CheckExtract;
use chrono::Duration;
use mongo_migration::{
    mongo_connection::MongoDatabaseOperate,
    mongo_models::bakery::mansion::operate::ToMansionOperate,
};
use resp_result::resp_try;
use tracing::{debug, instrument};

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
    #[instrument(skip(db), ret)]
    pub async fn save_mansion(
        db: MongoDatabaseOperate,
        CheckExtract(mid, ..): OptionMidCheckerPretreatment,
        CheckExtract(json, ..): MansionBodyCheckerPretreatment,
    ) -> MansionRResult<()> {
        resp_try(async move {
            let mid = mid.id;
            let data = json;

            match mid {
                Some(mid) => {
                    debug!(
                        mansion.id.provide = true,
                        mansion.saveMode = "Update"
                    );
                    db.mansion().update(mid, data).await?;
                }
                None => {
                    debug!(
                        mansion.id.provide = false,
                        mansion.saveMode = "Create"
                    );
                    db.mansion().create(data).await?;
                }
            }
            Ok(())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn get_mansion(
        db: MongoDatabaseOperate,
        CheckExtract(mid, ..): MidCheckerPretreatment,
    ) -> MansionRResult<ViewMansion> {
        resp_try(async {
            Ok(db.mansion().get_mansion_by_id(&mid.id).await?.into())
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn get_recent_id(
        db: MongoDatabaseOperate,
    ) -> MansionRResult<Vec<String>> {
        resp_try(async {
            Ok(db
                .mansion()
                .get_mansion_id_list_by_time(Duration::days(90))
                .await?)
        })
        .await
    }

    #[instrument(ret, skip(db))]
    pub async fn remove_mansion(
        db: MongoDatabaseOperate,
        CheckExtract(mid, ..): MidCheckerPretreatment,
    ) -> MansionRResult<()> {
        resp_try(async {
            db.mansion().delete(&mid.id).await?;
            Ok(())
        })
        .await
    }
}
