use chrono::Duration;
use persistence::{
    bakery::{
        mansion::ToMansion,
        models::mansion::{
            checked::{Mansion, Mid},
            models::{MansionId, ModelMansion},
        },
        ToBakery,
    },
    help_crates::tracing::debug,
    mongodb::MongoDatabaseOperate,
};
use tencent_cloud_server::cloud_manager::TcCloudManager;

use super::BakeryLogic;
use crate::{
    error::LogicResult,
    view::{BakeryTcCdnPath, MansionRecentPredictResp},
};

impl BakeryLogic {
    /// 获取最近日期的预测，没有就获取结果
    pub async fn recent_mansion_predict(
        mongo: MongoDatabaseOperate,
    ) -> LogicResult<Option<MansionRecentPredictResp>> {
        let mut recent_predict =
            mongo.bakery().mansion().get_recent_predict().await?;
        if recent_predict.is_none() {
            recent_predict =
                mongo.bakery().mansion().get_recent_result().await?;
        }
        Ok(recent_predict.map(|item| item.into()))
    }

    /// 保存大厦
    pub async fn save_mansion(
        mongo: MongoDatabaseOperate, tc_cloud: TcCloudManager,
        mid: Option<MansionId>, mansion: Mansion,
    ) -> LogicResult<()> {
        let mut paths = vec![
            BakeryTcCdnPath::MANSION_ID_PATH,
            BakeryTcCdnPath::RECENT_PREDICT_PATH,
            BakeryTcCdnPath::MANSION_INFO_PATH(&mansion.id.to_string())?,
        ];
        match mid {
            Some(mid) => {
                debug!(
                    mansion.id.provide = true,
                    mansion.saveMode = "Update"
                );
                mongo
                    .bakery()
                    .mansion()
                    .update(mid.clone(), mansion.clone())
                    .await?;
                if mansion.id != mid {
                    paths.push(BakeryTcCdnPath::MANSION_INFO_PATH(
                        &mid.to_string(),
                    )?);
                }
            }
            None => {
                debug!(
                    mansion.id.provide = false,
                    mansion.saveMode = "Create"
                );
                mongo.bakery().mansion().create(mansion.clone()).await?;
            }
        }
        tc_cloud.purge_urls_cache(&paths).await?;
        Ok(())
    }

    /// 获取最近90天大厦id
    pub async fn get_recent_id_by_90(
        mongo: MongoDatabaseOperate,
    ) -> LogicResult<Vec<String>> {
        Ok(mongo
            .bakery()
            .mansion()
            .get_mansion_id_list_by_time(Duration::days(90))
            .await?)
    }

    /// 获取所有大厦id
    pub async fn get_all_mansion_id(
        mongo: MongoDatabaseOperate,
    ) -> LogicResult<Vec<String>> {
        Ok(mongo.bakery().mansion().get_all_mansion_id_list().await?)
    }

    /// 根据id获取大厦
    pub async fn get_mansion(
        db: MongoDatabaseOperate, mid: Mid,
    ) -> LogicResult<ModelMansion> {
        Ok(db.bakery().mansion().get_mansion_by_id(&mid.id).await?)
    }

    /// 根据id删除大厦
    pub async fn remove_mansion(
        db: MongoDatabaseOperate, tc_cloud: TcCloudManager, mid: Mid,
    ) -> LogicResult<()> {
        db.bakery().mansion().delete(&mid.id).await?;

        let paths = [
            BakeryTcCdnPath::MANSION_ID_PATH,
            BakeryTcCdnPath::RECENT_PREDICT_PATH,
            BakeryTcCdnPath::MANSION_INFO_PATH(&mid.id.to_string())?,
        ];
        tc_cloud.purge_urls_cache(&paths).await?;
        Ok(())
    }
}
