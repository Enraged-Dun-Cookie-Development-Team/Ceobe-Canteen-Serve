use persistence::ceobe_operate::resource::countdown::CountdownType;
use serde::Serialize;
use persistence::ceobe_operate::models::resource::countdown;

use crate::utils::time_format::naive_date_time_format;

#[derive(Debug, Clone, Serialize)]
pub struct Countdown {
    #[serde(rename = "text")]
    message: String,
    #[serde(rename = "remark")]
    banner_info: String,
    countdown_type: Option<CountdownType>,
    #[serde(rename = "time")]
    countdown_end: String,
    start_time: String,
    over_time: String,
}
impl From<countdown::Model> for Countdown {
    fn from(
        countdown::Model {
            start_time,
            message,
            countdown_end,
            banner_info,
            over_time,
            countdown_type,
            ..
        }: countdown::Model,
    ) -> Self {
        Self {
            message,
            banner_info,
            countdown_type,
            countdown_end: naive_date_time_format(countdown_end),
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
        }
    }
}
