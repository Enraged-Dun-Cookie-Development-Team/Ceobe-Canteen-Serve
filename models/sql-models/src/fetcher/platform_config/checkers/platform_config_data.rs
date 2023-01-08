use checker::{
    check_obj,
    prefabs::{
        no_check::NoCheck, no_remainder_checker::NoRemainderChecker,
        option_checker::OptionChecker, str_len_checker::StrMaxCharLenChecker,
    },
};
use sea_orm::{IntoActiveModel, Set};
use typed_builder::TypedBuilder;

use super::{super::models::model_platform_config::ActiveModel, CheckError};

#[derive(Debug, TypedBuilder)]
pub struct FetcherPlatformConfig {
    pub id: Option<i32>,
    pub type_id: String,
    pub platform_name: String,
    pub min_request_interval: u64,
}

#[check_obj(
    uncheck = FetcherPlatformConfigUncheck,
    checked = FetcherPlatformConfig,
    error = CheckError
)]
#[derive(serde::Deserialize, Debug)]
pub struct FetcherPlatformConfigChecker {
    pub id: OptionChecker<NoCheck<i32>>,
    pub type_id: StrMaxCharLenChecker<String, 64>,
    pub platform_name: StrMaxCharLenChecker<String, 16>,
    pub min_request_interval: NoRemainderChecker<1000>,
}

impl IntoActiveModel<ActiveModel> for FetcherPlatformConfig {
    fn into_active_model(self) -> ActiveModel {
        let mut active = ActiveModel {
            min_request_interval: Set(self.min_request_interval),
            ..Default::default()
        };
        if let Some(id) = self.id.map(Set) {
            active.id = id
        } else {
            active.type_id = Set(self.type_id);
            active.platform_name = Set(self.platform_name);
        }

        active
    }
}
