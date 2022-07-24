use checker::{check_obj, prefabs::no_check::NoCheck};
use chrono::NaiveDateTime;
use range_limit::limits::max_limit::MaxRangeLimit;
use sea_orm::Set;
use typed_builder::TypedBuilder;

use crate::ceobe_operation::announcement::models::model_announcement;

use super::{
    CheckError,
};

#[derive(Debug, TypedBuilder)]
pub struct CeobeOperationAnnouncement {
    pub start_time: NaiveDateTime,
    pub over_time: NaiveDateTime,
    pub content: String,
    pub img_url: String,
    pub notice: bool,
}

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct CeobeOperationAnnouncementUncheck = CeobeOperationAnnouncementChecker > CeobeOperationAnnouncement{
        pub start_time: NoCheck<NaiveDateTime>,
        pub over_time: NoCheck<NaiveDateTime>,
        pub content: NoCheck<String>,
        pub img_url: MaxRangeLimit<String, 256>,
        pub notice: NoCheck<bool>
    }
    err : CheckError
}


impl model_announcement::ActiveModel {
    pub(in crate::ceobe_operation::announcement) fn from_announcement_data_with_order(
        CeobeOperationAnnouncement {
            start_time,
            over_time,
            content,
            img_url,
            notice,
        }: CeobeOperationAnnouncement,
        order: i32,
    ) -> Self {
        Self {
            start_time: Set(start_time),
            over_time :Set(over_time),
            content: Set(content),
            img_url: Set(img_url),
            order: Set(order),
            notice: Set(notice),
            ..Default::default()
        }
    }
}