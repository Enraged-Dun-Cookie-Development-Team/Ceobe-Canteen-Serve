use std::borrow::Cow;

use modify_cache::ModifyState;
use persistence::bakery::models::mansion::preludes::Predict;

mod convert;
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

impl MansionIds {
    pub(super) fn into_inner(this: Option<Self>) -> Option<Vec<String>> {
        this.map(|v| v.0)
    }
}

impl ModifyState for MansionIds {
    type Identify = Vec<String>;

    fn get_identify(&self) -> Cow<'_, Self::Identify> {
        Cow::Borrowed(&self.0)
    }
}
