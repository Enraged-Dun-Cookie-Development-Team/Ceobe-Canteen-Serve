use persistence::{
    bakery::models::mansion::preludes::{
        Daily, Info, Predict, RecentPredict,
    },
    help_crates::chrono::NaiveDate,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MansionRecentPredictResp {
    pub id: String,
    pub description: String,
    pub daily: ViewDaily,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ViewDaily {
    datetime: NaiveDate,
    info: Vec<ViewInfo>,
    content: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ViewInfo {
    forecast_status: Predict,
    forecast: String,
}
impl From<Info> for ViewInfo {
    fn from(Info { predict, forecast }: Info) -> Self {
        Self {
            forecast_status: predict,
            forecast,
        }
    }
}
impl From<Daily> for ViewDaily {
    fn from(
        Daily {
            date_time,
            content,
            info,
        }: Daily,
    ) -> Self {
        Self {
            datetime: date_time,
            info: info.into_iter().map(Into::into).collect(),
            content,
        }
    }
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
            daily: daily.into(),
        }
    }
}
