use std::borrow::Cow;

use modify_cache::ModifyState;
use orm_migrate::sql_models::ceobe_operation::video;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{
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

impl From<video::Model> for VideoItem {
    fn from(
        video::Model {
            start_time,
            over_time,
            title,
            author,
            video_link,
            cover_image,
            ..
        }: video::Model,
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

/// 用于请求头缓存信息生成
pub struct VideoItems(pub(super) Vec<VideoItem>);
impl VideoItems {
    pub(super) fn into_inner(this: Option<Self>) -> Option<Vec<VideoItem>> {
        this.map(|v| v.0)
    }
}
impl ModifyState for VideoItems {
    type Identify = Vec<VideoItem>;

    fn get_identify(&self) -> Cow<'_, Self::Identify> {
        Cow::Borrowed(&self.0)
    }
}
