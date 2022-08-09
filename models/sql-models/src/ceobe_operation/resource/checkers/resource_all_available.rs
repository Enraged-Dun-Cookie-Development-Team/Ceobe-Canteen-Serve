use checker::prefabs::date_time_format::DateTimeFormatChecker;
use chrono::NaiveDateTime;
use sea_orm::Set;
use serde::Deserialize;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::ceobe_operation::resource::models::model_resource;

#[derive(Debug, TypedBuilder)]
pub struct ResourceAllAvailableCheck {
    start_time: NaiveDateTime,
    over_time: NaiveDateTime,
}

#[checker::check_obj(
    uncheck = ResourceAllAvailableUncheck,
    checked = ResourceAllAvailableCheck,
    error = CheckError
)]
#[derive(Debug, Deserialize)]
pub struct ResourceAllAvailableChecker {
    start_time: DateTimeFormatChecker,
    over_time: DateTimeFormatChecker,
}

impl ResourceAllAvailableCheck {
    pub(super) fn into_active_with_create(self,now:NaiveDateTime) -> model_resource::ActiveModel {
        let mut active = model_resource::ActiveModel {
            start_time: Set(self.start_time),
            over_time: Set(self.over_time),
            ..Default::default()
        };
        active.now_create_with_time(now);

        active
    }
}
