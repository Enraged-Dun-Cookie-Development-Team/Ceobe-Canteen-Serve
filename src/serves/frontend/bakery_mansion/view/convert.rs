use mongo_migration::mongo_models::mansion_data::preludes::{
    Daily, Info, ModelMansion,
};

use super::{ViewDaily, ViewInfo, ViewMansionWithTime};

const TIME_FORMAT: &str = "%Y-%m-%d %T";

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
            datetime: date_time.format(TIME_FORMAT).to_string(),
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
            create_time: create_time
                .to_chrono()
                .format(TIME_FORMAT)
                .to_string(),
            modify_time: modify_time
                .to_chrono()
                .format(TIME_FORMAT)
                .to_string(),
        }
    }
}
