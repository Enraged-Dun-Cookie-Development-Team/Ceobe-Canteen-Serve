use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{
    models::sql::video::models::model_video,
    utils::time_format::naive_date_time_format,
};

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct VideoItem {
    pub start_time: String,
    pub over_time: String,
    pub title: String,
    pub author: String,
    pub video_link: String,
    #[serde(rename = "cover_img")]
    pub cover_image: String,
}

impl From<model_video::Model> for VideoItem {
    fn from(
        model_video::Model {
            start_time,
            over_time,
            title,
            author,
            video_link,
            cover_image,
            ..
        }: model_video::Model,
    ) -> Self {
        Self {
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
            title,
            author,
            video_link,
            cover_image,
        }
    }
}
