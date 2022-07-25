use checker::{
    check_obj,
    prefabs::{date_time_format::DateTimeFormatChecker, no_check::NoCheck},
};
use chrono::NaiveDateTime;
use range_limit::limits::max_limit::MaxRangeLimit;
use sea_orm::Set;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::ceobe_operation::announcement::models::model_announcement;

#[derive(Debug, TypedBuilder)]
pub struct CeobeOpAnnouncement {
    pub start_time: NaiveDateTime,
    pub over_time: NaiveDateTime,
    pub content: String,
    pub img_url: String,
    pub notice: bool,
}

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct CeobeOpAnnouncementUncheck = CeobeOpAnnouncementChecker > CeobeOpAnnouncement{
        pub start_time: DateTimeFormatChecker,
        pub over_time: DateTimeFormatChecker,
        pub content: NoCheck<String>,
        pub img_url: MaxRangeLimit<String, 256>,
        pub notice: NoCheck<bool>
    }
    err : CheckError
}

impl model_announcement::ActiveModel {
    pub(in crate::ceobe_operation::announcement) fn from_announcement_data_with_order(
        CeobeOpAnnouncement {
            start_time,
            over_time,
            content,
            img_url,
            notice,
        }: CeobeOpAnnouncement,
        order: i32,
    ) -> Self {
        Self {
            start_time: Set(start_time),
            over_time: Set(over_time),
            content: Set(content),
            img_url: Set(img_url),
            order: Set(order),
            notice: Set(notice),
            ..Default::default()
        }
    }
}
