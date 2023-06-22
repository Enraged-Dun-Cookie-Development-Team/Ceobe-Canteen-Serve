use bakery::{ToBakery, mansion::ToMansion};
use db_ops_prelude::{mongo_connection::MongoDatabaseOperate, mongo_models::bakery::mansion::preludes::RecentPredict};

use crate::{error::LogicResult, view::MansionRecentPredictResp};

use super::BakeryLogic;



impl BakeryLogic {
    /// 获取最近日期的预测，没有就获取结果
    pub async fn recent_mansion_predict(
        mongo: MongoDatabaseOperate,
    ) -> LogicResult<Option<MansionRecentPredictResp>> {
        let mut recent_predict = mongo.bakery().mansion().get_recent_predict().await?;
        if recent_predict.is_none() {
            recent_predict = mongo.bakery().mansion().get_recent_result().await?;
        }
        Ok(recent_predict.map(|item| item.into()))
    }
}