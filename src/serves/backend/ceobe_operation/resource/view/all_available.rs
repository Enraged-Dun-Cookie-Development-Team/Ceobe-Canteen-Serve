use orm_migrate::sql_models::ceobe_operation::resource::models::model_resource;
use serde::Serialize;

use crate::utils::time_format::naive_date_time_format;

#[derive(Debug, Clone, Serialize)]
pub struct AllAvailable {
    start_time: String,
    over_time: String,
}
impl From<model_resource::ResourceAllAvailable> for AllAvailable {
    fn from(
        model_resource::ResourceAllAvailable {
            over_time,
            start_time,
            ..
        }: model_resource::ResourceAllAvailable,
    ) -> Self {
        Self {
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
        }
    }
}
