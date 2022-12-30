use checker::{
    check_obj,
    prefabs::{no_check::NoCheck, option_checker::OptionChecker},
};
use range_limit::{limits::max_limit::MaxLimit, RangeBoundLimit};
use sea_orm::{ActiveValue::NotSet, Set};
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::fetcher::platform_config::models::model_platform_config;

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;

#[derive(Debug, TypedBuilder)]
pub struct FetcherPlatformConfig {
    pub id: Option<i32>,
    pub type_id: String,
    pub platform_name: String,
    pub min_request_interval: i32,
}

#[check_obj(
    uncheck = FetcherPlatformConfigUncheck,
    checked = FetcherPlatformConfig,
    error = CheckError
)]
#[derive(serde::Deserialize, Debug)]
pub struct FetcherPlatformConfigChecker {
    pub id: OptionChecker<NoCheck<i32>>,
    pub type_id: MaxLimitString<64>,
    pub platform_name: MaxLimitString<64>,
    pub min_request_interval: NoCheck<i32>, /* TODO: 检查时候能被1000整除 */
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
        Self {
            id: match id {
                Some(id) => Set(id),
                None => NotSet,
            },
            type_id: match id {
                Some(_) => NotSet,
                None => Set(type_id),
            },
            platform_name: match id {
                Some(_) => NotSet,
                None => Set(platform_name),
            },
            min_request_interval: Set(min_request_interval),
        }
    }
}
