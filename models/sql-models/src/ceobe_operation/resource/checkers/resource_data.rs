use checker::{
    check_obj, prefabs::collect_checkers::iter_checkers::IntoIterChecker,
};
use chrono::NaiveDateTime;
use typed_builder::TypedBuilder;

use super::{
    countdown::{CountdownCheck, CountdownChecker, CountdownUncheck},
    resource_all_available::{
        ResourceAllAvailableCheck, ResourceAllAvailableChecker,
    },
    CheckError,
};
use crate::ceobe_operation::resource::models::model_resource;

#[derive(Debug, TypedBuilder)]
pub struct CeobeOperationResource {
    pub resource_all_available: ResourceAllAvailableCheck,
    pub countdown: Vec<CountdownCheck>,
}

#[check_obj(
    uncheck = CeobeOperationResourceUncheck,
    checked = CeobeOperationResource,
    error = CheckError
)]
pub struct CeobeOperationResourceChecker {
    resource_all_available: ResourceAllAvailableChecker,
    countdown: IntoIterChecker<
        Vec<CountdownUncheck>,
        CountdownChecker,
        Vec<CountdownCheck>,
    >,
}

impl CeobeOperationResource {
    pub fn into_active_list(
        self, now: NaiveDateTime,
    ) -> Vec<model_resource::ActiveModel> {
        let size = self.countdown.len() + 1;

        self.countdown
            .into_iter()
            .map(|countdown| {
                CountdownCheck::into_active_with_create(countdown, now)
            })
            .fold(
                {
                    let mut vec = Vec::with_capacity(size);
                    vec.push(
                        self.resource_all_available
                            .into_active_with_create(now),
                    );
                    vec
                },
                |mut vec, countdown| {
                    vec.push(countdown);
                    vec
                },
            )
    }
}
