mod convert;
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

