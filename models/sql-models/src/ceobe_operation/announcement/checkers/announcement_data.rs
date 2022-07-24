use checker::{check_obj, prefabs::no_check::NoCheck};
use chrono::NaiveDateTime;
use range_limit::limits::max_limit::MaxRangeLimit;
use typed_builder::TypedBuilder;

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