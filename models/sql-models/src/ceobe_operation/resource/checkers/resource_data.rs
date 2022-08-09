use checker::{
    check_obj,
    prefabs::{
        collect_checkers::iter_checkers::IntoIterChecker,
        option_checker::OptionChecker,
    },
};
use chrono::NaiveDateTime;
use serde::Deserialize;
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
    pub resource_all_available: Option<ResourceAllAvailableCheck>,
    pub countdown: Option<Vec<CountdownCheck>>,
}

#[check_obj(
    uncheck = CeobeOperationResourceUncheck,
    checked = CeobeOperationResource,
    error = CheckError
)]
#[derive(Debug, Deserialize)]
pub struct CeobeOperationResourceChecker {
    resource_all_available: OptionChecker<ResourceAllAvailableChecker>,
    countdown: OptionChecker<
        IntoIterChecker<
            Vec<CountdownUncheck>,
            CountdownChecker,
            Vec<CountdownCheck>,
        >,
    >,
}

impl CeobeOperationResource {
    pub fn into_active_list(
        self, now: NaiveDateTime,
    ) -> Vec<model_resource::ActiveModel> {
        let size = if let Some(ref countdown) = self.countdown {
            countdown.len()
        }
        else {
            0
        } + if self.resource_all_available.is_some() {
            1
        }
        else {
            0
        };
        match (self.countdown, self.resource_all_available) {
            (None, None) => Vec::new(),
            (None, Some(raa)) => vec![raa.into_active_with_create(now)],
            (Some(countdown), None) => {
                countdown
                    .into_iter()
                    .map(|c| CountdownCheck::into_active_with_create(c, now))
                    .collect()
            }
            (Some(countdown), Some(resource_all_available)) => {
                countdown
                    .into_iter()
                    .map(|countdown| {
                        CountdownCheck::into_active_with_create(
                            countdown, now,
                        )
                    })
                    .fold(
                        {
                            let mut vec = Vec::with_capacity(size);
                            vec.push(
                                resource_all_available
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
    }
}
