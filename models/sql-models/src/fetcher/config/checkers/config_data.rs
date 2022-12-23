
use checker::{check_obj, prefabs::no_check::NoCheck};
use range_limit::{limits::max_limit::MaxLimit, RangeBoundLimit};
use sea_orm::Set;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::fetcher::config::models::model_config;

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;

#[derive(Debug, TypedBuilder)]
pub struct FetcherConfig {
    pub live_number: i8,
    pub fetcher_count: i8,
    pub group_name: String,
    pub platform: String,
    pub datasource_id: i32,
    pub interval: i32,
    pub interval_by_time_range: String,
}

#[check_obj(
    uncheck = FetcherConfigUncheck,
    checked = FetcherConfig,
    error = CheckError
)]
#[derive(Debug,serde::Deserialize)]
pub struct FetcherConfigChecker {
    pub live_number: NoCheck<i8>,
    pub fetcher_count: NoCheck<i8>,
    pub group_name: MaxLimitString<16>,
    pub platform: MaxLimitString<64>,
    pub datasource_id: NoCheck<i32>,
    pub interval: NoCheck<i32>,
    pub interval_by_time_range: NoCheck<String>,
}

impl model_config::ActiveModel {
    pub(in crate::fetcher::config) fn fetcher_config_into_active_model (
        FetcherConfig {
            live_number,
            fetcher_count,
            group_name,
            platform,
            datasource_id,
            interval,
            interval_by_time_range,
        }: FetcherConfig
    )-> Self {
        Self {
            live_number: Set(live_number),
            fetcher_count: Set(fetcher_count),
            group_name: Set(group_name),
            platform: Set(platform),
            datasource_id: Set(datasource_id),
            interval: Set(interval),
            interval_by_time_range: Set(interval_by_time_range),
            ..Default::default()
        }
    }
}
