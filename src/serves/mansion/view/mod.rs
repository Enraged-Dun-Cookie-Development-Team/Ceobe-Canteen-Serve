use crate::models::mansion::preludes::*;


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
            id: id.to_string(),
            description,
            cvlink: link,
            fraction: fraction as u8,
            daily: daily.into_iter().map(Into::into).collect(),
        }
    }
}
impl Into<ViewMansion> for ModelMansion {
    fn into(self) -> ViewMansion {
        let ModelMansion {
            id,
            description,
            cvlink,
            fraction,
            daily,
            ..
        } = self;
        ViewMansion {
            id: id.to_string(),
            description,
            cvlink,
            fraction,
            daily: daily.into_iter().map(Into::into).collect(),
        }
    }
}
