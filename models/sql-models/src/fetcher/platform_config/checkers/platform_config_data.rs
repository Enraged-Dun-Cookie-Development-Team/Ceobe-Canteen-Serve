use checker::{
    check_obj,
    prefabs::{
        no_check::NoCheck, no_remainder_checker::NoRemainderChecker,
        option_checker::OptionChecker, str_len_checker::StrMaxCharLenChecker,
    },
};
use sea_orm::Set;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::fetcher::platform_config::models::model_platform_config;

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

impl model_platform_config::ActiveModel {
    pub(in crate::fetcher::platform_config) fn platform_config_into_active_model(
        FetcherPlatformConfig {
            id,
            type_id,
            platform_name,
            min_request_interval,
        }: FetcherPlatformConfig,
    ) -> Self {
        let mut this = Self::default();
        if let Some(id) = id {
            this.id = Set(id);
        }
        else {
            this.type_id = Set(type_id);
            this.platform_name = Set(platform_name);
        }
        this.min_request_interval = Set(min_request_interval);
        this
    }
}
