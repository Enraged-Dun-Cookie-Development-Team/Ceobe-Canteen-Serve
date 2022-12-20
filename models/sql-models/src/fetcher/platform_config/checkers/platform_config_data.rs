
use checker::{check_obj, prefabs::no_check::NoCheck};
use range_limit::{RangeBoundLimit, limits::max_limit::MaxLimit};
use sea_orm::Set;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::fetcher::platform_config::models::model_platform_config;

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;

#[derive(Debug, TypedBuilder)]
pub struct FetcherPlatformConfig {
    pub type_id: String,
    pub platform_name: String,
    pub min_request_interval: i32
}

#[check_obj(
    uncheck = FetcherPlatformConfigUncheck,
    checked = FetcherPlatformConfig,
    error = CheckError
)]
#[derive(serde::Deserialize, Debug)]
pub struct FetcherPlatformConfigChecker {
    pub type_id: MaxLimitString<64>,
    pub platform_name: MaxLimitString<64>,
    pub min_request_interval: NoCheck<i32>
}

impl model_platform_config::ActiveModel {
    pub(in crate::fetcher::platform_config) fn platform_config_into_active_model(
        FetcherPlatformConfig {
            type_id,
            platform_name,
            min_request_interval,
        }: FetcherPlatformConfig,
    ) -> Self {
        Self {
            type_id: Set(type_id),
            platform_name: Set(platform_name),
            min_request_interval: Set(min_request_interval),
            ..Default::default()
        }
    }
}
