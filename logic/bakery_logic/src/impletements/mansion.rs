use chrono::Duration;
use persistence::{
    bakery::{mansion::ToMansion, models::mansion::{checked::{Mansion, Mid}, models::{MansionId, ModelMansion}}, ToBakery}, help_crates::tracing::debug, mongodb::MongoDatabaseOperate
};

use super::BakeryLogic;
use crate::{error::LogicResult, view::MansionRecentPredictResp};

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
        mongo: MongoDatabaseOperate, mid: Option<MansionId>, mansion: Mansion
    ) -> LogicResult<()> {
        match mid {
            Some(mid) => {
                debug!(
                    mansion.id.provide = true,
                    mansion.saveMode = "Update"
                );
                mongo.bakery().mansion().update(mid, mansion).await?;
            }
            None => {
                debug!(
                    mansion.id.provide = false,
                    mansion.saveMode = "Create"
                );
                mongo.bakery().mansion().create(mansion).await?;
            }
        }
        Ok(())
    }

    /// 获取最近90天大厦id
    pub async fn get_recent_id_by_90(
        mongo: MongoDatabaseOperate
    ) -> LogicResult<Vec<String>> {
        Ok(mongo
            .bakery()
            .mansion()
            .get_mansion_id_list_by_time(Duration::days(90))
            .await?)
    }

    /// 获取所有大厦id
    pub async fn get_all_mansion_id(
        mongo: MongoDatabaseOperate
    ) -> LogicResult<Vec<String>> {
        Ok(mongo
            .bakery()
            .mansion()
            .get_all_mansion_id_list()
            .await?)
    }

    /// 根据id获取大厦
    pub async fn get_mansion(
        db: MongoDatabaseOperate,
        mid: Mid
    ) -> LogicResult<ModelMansion> {
            Ok(db
                .bakery()
                .mansion()
                .get_mansion_by_id(&mid.id)
                .await?)
    }

    /// 根据id删除大厦
    pub async fn remove_mansion(
        db: MongoDatabaseOperate,
        mid: Mid
    ) -> LogicResult<()> {
        db.bakery().mansion().delete(&mid.id).await?;
        Ok(())
    }


}
