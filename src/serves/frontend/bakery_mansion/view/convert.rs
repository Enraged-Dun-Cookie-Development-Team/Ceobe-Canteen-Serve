use persistence::bakery::models::mansion::preludes::{
    Daily, Info, ModelMansion,
};

use super::{ViewDaily, ViewInfo, ViewMansionWithTime};
use crate::utils::time_format::{bson_date_time_format, naive_date_format};

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
            datetime: naive_date_format(date_time),
            info: info.into_iter().map(Into::into).collect(),
            content,
        }
    }
}

impl From<ModelMansion> for ViewMansionWithTime {
    fn from(val: ModelMansion) -> Self {
        let ModelMansion {
            id,
            description,
            cvlink,
            fraction,
            daily,
            create_time,
            modify_time,
        } = val;
        ViewMansionWithTime {
            id: id.to_string(),
            description,
            cvlink,
            fraction,
            daily: daily.into_iter().map(Into::into).collect(),
            create_time: bson_date_time_format(create_time),
            modify_time: bson_date_time_format(modify_time),
        }
    }
}
