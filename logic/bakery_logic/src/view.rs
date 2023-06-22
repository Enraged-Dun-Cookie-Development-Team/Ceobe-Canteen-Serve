use db_ops_prelude::mongo_models::bakery::mansion::preludes::{Daily, RecentPredict};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MansionRecentPredictResp {
    pub id: String,
    pub description: String,
    pub daily: Daily,
}

impl Into<MansionRecentPredictResp> for RecentPredict {
    fn into(self) -> MansionRecentPredictResp {
        let RecentPredict{
            id,
            description,
            daily,
        } = self;
        MansionRecentPredictResp {
            id: id.to_string(),
            description,
            daily,
        }
    }
}