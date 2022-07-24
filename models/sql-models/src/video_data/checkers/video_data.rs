use checker::{check_obj, prefabs::no_check::NoCheck};
use chrono::NaiveDateTime;
use range_limit::limits::max_limit::MaxRangeLimit;
use typed_builder::TypedBuilder;

use super::{
    bv::{Bv, BvChecker},
    VideoDataCheckError,
};

#[derive(Debug, TypedBuilder)]
pub struct VideoData {
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
    pub struct VideoDataUncheck = VideoDataChecker > VideoData{
        pub bv: BvChecker,
        pub start_time: NoCheck<NaiveDateTime>,
        pub over_time: NoCheck<NaiveDateTime>,
        pub title: MaxRangeLimit<String, 256>,
        pub author: MaxRangeLimit<String, 128>,
        pub video_link: MaxRangeLimit<String, 256>,
        pub cover_image: MaxRangeLimit<String, 256>
    }
    err : VideoDataCheckError
}
