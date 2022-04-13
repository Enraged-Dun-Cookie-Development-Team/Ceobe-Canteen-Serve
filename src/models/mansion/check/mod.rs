use chrono::NaiveDate;
use sub_model_derive::SubModel;

use super::mongo_db::{MansionId, Predict};

crate::quick_struct! {
    /// 反序列化器，必须有mansion Id
    pub Mid{
        id: MansionId
    }
    /// 反序列化器，可能有 mansion Id
    pub OptionMid{
        id: Option<MansionId>
    }

    pub Mansion{
        id: MansionId
        #[serde(alias="cvlink")]
        link: String
        description: String
        fraction: i16
        daily: Vec<Daily>
    }

    /// Mansion Daily Checked Info
    #[derive(SubModel)]
    #[sub_model(
        all("ViewDaily")
    )]
    pub Daily{
        #[serde(rename = "datetime")]
        #[sub_model(
            having(
                for = "ViewDaily",
                rename = "datetime",
                to_type(ty = "String", by = "date_time_mapping")
            )
        )]
        date_time: NaiveDate
        content: String
        #[sub_model(
            having(
                for = "ViewDaily",
                to_type(ty = "Vec<ViewInfo>", by = "info_mapping")
            )
        )]
        info: Vec<Info>
    }
    /// Mansion Info Checked Model
    #[derive(SubModel)]
    #[sub_model(
        all("ViewInfo")
    )]
    pub Info{
        #[sub_model(
            having(
                for = "ViewInfo",
                rename = "is_true",
                extra(serde(rename = "isTrue"))
            )
        )]
        predict: Predict
        forecast: String
    }
}

fn date_time_mapping(time: NaiveDate) -> String {
    time.format("%Y-%m-%d").to_string()
}

fn info_mapping(info: Vec<Info>) -> Vec<ViewInfo> {
    info.into_iter().map(Into::into).collect()
}
