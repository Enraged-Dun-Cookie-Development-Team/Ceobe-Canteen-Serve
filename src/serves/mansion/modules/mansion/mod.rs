// use chrono::NaiveDate;

// use crate::serves::mansion::db_ops;

mod checkers;

pub use checkers::*;

crate::quick_struct! {
    pub ViewMansion{
        id:String
        description:String
        cvlink:String
        fraction:u8
        daily:Vec<ViewDaily>
    }

    pub ViewDaily{
        datetime:String
        info:Vec<ViewInfo>
        content:String
    }

    pub ViewInfo{
        #[serde(rename="isTrue")]
        is_true:Predict
        forecast:String
    }
}

impl From<Info> for ViewInfo {
    fn from(Info { predict, forecast }: Info) -> Self {
        Self {
            is_true: predict,
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
            datetime: date_time.format("%Y-%m-%d").to_string(),
            info: info.into_iter().map(Into::into).collect(),
            content,
        }
    }
}

impl From<Mansion> for ViewMansion {
    fn from(
        Mansion {
            id,
            link,
            description,
            fraction,
            daily,
        }: Mansion,
    ) -> Self {
        Self {
            id: format!("{}.{}", id.0, id.1),
            description,
            cvlink: link,
            fraction: fraction as u8,
            daily: daily.into_iter().map(Into::into).collect(),
        }
    }
}
