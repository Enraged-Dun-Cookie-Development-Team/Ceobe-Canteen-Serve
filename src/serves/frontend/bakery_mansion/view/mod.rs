mod convert;
use std::borrow::Cow;

use cache_verify::ModifyState;
use chrono::NaiveDateTime;

use crate::models::mansion::preludes::*;

crate::quick_struct! {
    pub ViewMansionWithTime {
        id:String
        description:String
        #[serde(rename="cv_link")]
        cvlink:String
        create_time: String
        modify_time: String
        fraction:u8
        daily:Vec<ViewDaily>
    }

    pub ViewDaily{
        datetime:String
        info:Vec<ViewInfo>
        content:String
    }

    pub ViewInfo{
        forecast_status:Predict
        forecast:String
    }
}

pub struct MansionIds(pub(super) Vec<String>);

impl ModifyState for MansionIds {
    type Identify = Vec<String>;

    fn get_last_modify_time(&self) -> Cow<'_, chrono::NaiveDateTime> {
        Cow::Owned(NaiveDateTime::from_timestamp(0, 0))
    }

    fn get_identify(&self) -> Cow<'_, Self::Identify> {
        Cow::Borrowed(&self.0)
    }
}
