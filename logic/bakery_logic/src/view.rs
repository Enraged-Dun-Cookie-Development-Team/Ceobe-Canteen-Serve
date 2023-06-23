use db_ops_prelude::mongo_models::bakery::mansion::preludes::{
    Daily, RecentPredict,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MansionRecentPredictResp {
    pub id: String,
    pub description: String,
    pub daily: Daily,
}

impl From<RecentPredict> for MansionRecentPredictResp {
    fn from(val: RecentPredict) -> Self {
        let RecentPredict {
            id,
            description,
            daily,
        } = val;
        Self {
            id: id.to_string(),
            description,
            daily,
        }
    }
}
