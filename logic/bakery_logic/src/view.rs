use persistence::{
    bakery::models::mansion::{checked::Mansion, models::ModelMansion, preludes::{
        Daily, Info, Predict, RecentPredict,
    }},
    help_crates::{bson_date_time_format, chrono::NaiveDate},
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MansionRecentPredictResp {
    pub id: String,
    pub description: String,
    pub daily: ViewDaily,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ViewDaily {
    datetime: NaiveDate,
    info: Vec<ViewInfo>,
    content: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ViewInfo {
    forecast_status: Predict,
    forecast: String,
}
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
            datetime: date_time,
            info: info.into_iter().map(Into::into).collect(),
            content,
        }
    }
}
impl From<RecentPredict> for MansionRecentPredictResp {
    fn from(val: RecentPredict) -> Self {
        let RecentPredict {
            id,
            description,
            daily,
        } = val;
        Self {
            id: id.to_string(),
            description,
            daily: daily.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct MansionResp{
    pub id:String,
    pub description:String,
    #[serde(rename="cv_link")]
    pub cvlink:String,
    pub fraction:u8,
    pub daily:Vec<ViewDaily>,
}


impl From<Mansion> for MansionResp {
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
impl From<ModelMansion> for MansionResp {
    fn from(val: ModelMansion) -> Self {
        let ModelMansion {
            id,
            description,
            cvlink,
            fraction,
            daily,
            ..
        } = val;
        MansionResp {
            id: id.to_string(),
            description,
            cvlink,
            fraction,
            daily: daily.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct MansionWithTimeResp {
    pub id:String,
    pub description:String,
    #[serde(rename="cv_link")]
    pub cvlink:String,
    pub create_time: String,
    pub modify_time: String,
    pub fraction:u8,
    pub daily:Vec<ViewDaily>,
}

impl From<ModelMansion> for MansionWithTimeResp {
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
        MansionWithTimeResp {
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
