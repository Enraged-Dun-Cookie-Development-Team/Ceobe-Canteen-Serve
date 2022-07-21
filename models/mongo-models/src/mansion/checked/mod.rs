use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::mongo_db::{MansionId, Predict};

/// 反序列化器，必须有mansion Id
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Mid {
    pub id: MansionId,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
/// 反序列化器，可能有 mansion Id
pub struct OptionMid {
    pub id: Option<MansionId>,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Mansion {
    pub id: MansionId,
    #[serde(alias = "cv_link")]
    pub link: String,
    pub description: String,
    pub fraction: i16,
    pub daily: Vec<Daily>,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
/// Mansion Daily Checked Info
pub struct Daily {
    #[serde(rename = "datetime")]
    pub date_time: NaiveDate,
    pub content: String,
    pub info: Vec<Info>,
}
/// Mansion Info Checked Model
#[derive(
    Debug, Clone, Serialize, Deserialize, typed_builder::TypedBuilder,
)]
pub struct Info {
    #[serde(alias = "forecast_status")]
    pub predict: Predict,
    pub forecast: String,
}
