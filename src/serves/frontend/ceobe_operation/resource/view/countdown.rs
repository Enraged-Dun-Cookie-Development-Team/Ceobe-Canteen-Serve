use orm_migrate::sql_models::ceobe_operation::resource::models::model_resource;
use serde::Serialize;

use crate::utils::time_format::naive_date_time_format;

#[derive(Debug, Clone, Serialize)]
pub struct Countdown {
    #[serde(rename = "text")]
    message: String,
    #[serde(rename = "remark")]
    banner_info: String,
    #[serde(rename = "time")]
    countdown_end: String,
    start_time: String,
    over_time: String,
}
impl From<model_resource::Countdown> for Countdown {
    fn from(
        model_resource::Countdown {
            start_time,
            message,
            countdown_end,
            banner_info,
            over_time,
            ..
        }: model_resource::Countdown,
    ) -> Self {
        Self {
            message,
            banner_info,
            countdown_end: naive_date_time_format(countdown_end),
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
        }
    }
}
