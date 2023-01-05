use checker::prefabs::{date_time_format::DateTimeFormatChecker, str_len_checker::{StrMaxCharLenChecker}};
use chrono::NaiveDateTime;
use sea_orm::Set;
use serde::Deserialize;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::ceobe_operation::resource::models::{
    model_resource::ActiveModel, resource_type::ResourceType,
};

#[derive(Debug, TypedBuilder)]
pub struct CountdownCheck {
    message: String,
    banner_info: String,
    countdown_end: NaiveDateTime,
    start_time: NaiveDateTime,
    over_time: NaiveDateTime,
}

#[checker::check_gen(
    uncheck = CountdownUncheck,
    checked = CountdownCheck,
    error = CheckError
)]
#[derive(Debug, Deserialize)]
pub struct CountdownChecker {
    #[serde(alias = "text")]
    message: StrMaxCharLenChecker<String, 255>,
    #[serde(alias = "remark")]
    banner_info: StrMaxCharLenChecker<String, 255>,
    #[serde(alias = "time")]
    countdown_end: DateTimeFormatChecker,
    start_time: DateTimeFormatChecker,
    over_time: DateTimeFormatChecker,
}

impl CountdownCheck {
    pub(super) fn into_active_with_create(
        Self {
            message,
            banner_info,
            countdown_end,
            start_time,
            over_time,
        }: Self,
        now: NaiveDateTime,
    ) -> ActiveModel {
        let mut active = ActiveModel {
            ty: Set(ResourceType::Countdown),
            message: Set(message),
            banner_info: Set(banner_info),
            countdown_end: Set(countdown_end),
            start_time: Set(start_time),
            over_time: Set(over_time),
            ..Default::default()
        };
        active.now_create_with_time(now);
        active
    }
}
