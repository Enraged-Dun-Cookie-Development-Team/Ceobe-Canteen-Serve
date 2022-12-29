
use checker::{check_obj, prefabs::{no_check::NoCheck, option_checker::OptionChecker, collect_checkers::iter_checkers::IntoIterChecker}};
use range_limit::{limits::max_limit::MaxLimit, RangeBoundLimit};
use sea_orm::{Set, ActiveValue::NotSet};
use serde_json::Value;
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
    pub interval: Option<i32>,
    pub interval_by_time_range: Value,
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
    pub interval: OptionChecker<NoCheck<i32>>,
    pub interval_by_time_range: NoCheck<Value>,
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
            interval: match interval  {
                Some(value) => Set(Some(value)),
                None => Set(None)
            },
            interval_by_time_range: match interval_by_time_range  {
                Value::Null => Set(None),
                _ => Set(Some(interval_by_time_range.to_string()))
            },
            ..Default::default()
        }
    }
}

// 用于验证FetcherConfig数组
pub type FetcherConfigVecChecker = IntoIterChecker<Vec<FetcherConfigUncheck>, FetcherConfigChecker, Vec<FetcherConfig>>;
