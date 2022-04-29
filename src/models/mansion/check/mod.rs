use chrono::NaiveDate;

use super::mongo_db::{MansionId, Predict};


crate::quick_struct!{
    /// 反序列化器，必须有mansion Id
    pub Mid{
        id:MansionId
    }
    /// 反序列化器，可能有 mansion Id
    pub OptionMid{
        id:Option<MansionId>
    }

    pub Mansion{
        id:MansionId
        #[serde(alias="cv_link")]
        link:String
        description:String
        fraction:i16
        daily:Vec<Daily>
    }
    
    /// Mansion Daily Checked Info
    pub Daily{
        #[serde(rename="datetime")]
        date_time:NaiveDate
        content:String
        info:Vec<Info>
    }
    /// Mansion Info Checked Model
    pub Info{
        predict:Predict
        forecast:String
    }
}

