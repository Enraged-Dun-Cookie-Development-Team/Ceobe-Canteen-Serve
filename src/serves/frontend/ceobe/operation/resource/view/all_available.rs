use persistence::ceobe_operate::models::resource::all_available;
use serde::Serialize;

use crate::utils::time_format::naive_date_time_format;

#[derive(Debug, Clone, Serialize)]
pub struct AllAvailable {
    start_time: String,
    over_time: String,
}
impl From<all_available::Model> for AllAvailable {
    fn from(
        all_available::Model {
            over_time,
            start_time,
            ..
        }: all_available::Model,
    ) -> Self {
        Self {
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
        }
    }
}
