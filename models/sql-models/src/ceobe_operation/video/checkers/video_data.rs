use checker::{check_obj, prefabs::date_time_format::DateTimeFormatChecker};
use chrono::NaiveDateTime;
use range_limit::limits::max_limit::MaxRangeLimit;
use sea_orm::Set;
use typed_builder::TypedBuilder;

use super::{
    bv::{Bv, BvChecker},
    CheckError,
};
use crate::ceobe_operation::video::models::model_video;

#[derive(Debug, TypedBuilder)]
pub struct CeobeOperationVideo {
    pub bv: Bv,
    pub start_time: NaiveDateTime,
    pub over_time: NaiveDateTime,
    pub title: String,
    pub author: String,
    pub video_link: String,
    pub cover_image: String,
}

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct CeobeOperationVideoUncheck = CeobeOperationVideoChecker > CeobeOperationVideo{
        pub bv: BvChecker,
        pub start_time: DateTimeFormatChecker,
        pub over_time: DateTimeFormatChecker,
        pub title: MaxRangeLimit<String, 256>,
        pub author: MaxRangeLimit<String, 128>,
        pub video_link: MaxRangeLimit<String, 256>,
        pub cover_image: MaxRangeLimit<String, 256>
    }
    err : CheckError
}

impl model_video::ActiveModel {
    pub(in crate::ceobe_operation::video) fn from_video_data_with_order(
        CeobeOperationVideo {
            bv,
            start_time,
            over_time,
            title,
            author,
            video_link,
            cover_image,
        }: CeobeOperationVideo,
        order: i32,
    ) -> Self {
        Self {
            bv: Set(bv.to_string()),
            order: Set(order),
            start_time: Set(start_time),
            over_time: Set(over_time),
            title: Set(title),
            author: Set(author),
            video_link: Set(video_link),
            cover_image: Set(cover_image),
            ..Default::default()
        }
    }

    pub(in crate::ceobe_operation::video) fn update_with_video_and_order(
        &mut self,
        CeobeOperationVideo {
            bv: _,
            start_time,
            over_time,
            title,
            author,
            video_link,
            cover_image,
        }: CeobeOperationVideo,
        order: i32,
    ) {
        self.order = Set(order);
        self.start_time = Set(start_time);
        self.over_time = Set(over_time);
        self.title = Set(title);
        self.author = Set(author);
        self.video_link = Set(video_link);
        self.cover_image = Set(cover_image);
        self.soft_recover();
    }
}
