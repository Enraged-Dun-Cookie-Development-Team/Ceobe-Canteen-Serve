use checker::{
    check_obj,
    prefabs::{
        collect_checkers::iter_checkers::IntoIterChecker, no_check::NoCheck,
        no_remainder_checker::NoRemainderChecker,
        option_checker::OptionChecker, str_len_checker::StrMaxCharLenChecker,
    },
};
use sea_orm::{IntoActiveModel, Set};
use serde::Deserialize;
use serde_json::Value;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::fetcher::config::models::model_config;

#[derive(Debug, TypedBuilder)]
pub struct FetcherConfig {
    pub live_number: i8,
    pub fetcher_count: FetcherCount,
    pub group_name: String,
    pub platform: String,
    pub datasource_id: i32,
    pub interval: Option<u64>,
    pub interval_by_time_range: Value,
}

impl FetcherConfig {
    pub fn get_platform_type_id(&self) -> &str { &self.platform }
}
use ::checker::ToCheckRequire;
#[check_obj(
    uncheck = FetcherConfigUncheck,
    checked = FetcherConfig,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize, typed_builder::TypedBuilder)]
pub struct FetcherConfigChecker {
    #[builder(setter(
        transform = |number:i8| ToCheckRequire::require_check(number)
    ))]
    pub live_number: NoCheck<i8>,
    #[builder(setter(
        transform = |count:usize| ToCheckRequire::require_check(FetcherCount::new(count as i8))
    ))]
    pub fetcher_count: NoCheck<FetcherCount>,
    #[builder(setter(
        transform = |name:String| ToCheckRequire::require_check(name)
    ))]
    pub group_name: StrMaxCharLenChecker<String, 32>,
    #[builder(setter(
        transform = |platform:String| ToCheckRequire::require_check(platform)
    ))]
    pub platform: StrMaxCharLenChecker<String, 64>,
    #[builder(setter(
        transform = |data_source_id:i32| ToCheckRequire::require_check(data_source_id)
    ))]
    pub datasource_id: NoCheck<i32>,
    #[builder(setter(
        transform = |interval:Option<u64>| ToCheckRequire::require_check(interval)
    ))]
    pub interval: OptionChecker<NoRemainderChecker<1000>>,
    #[builder(setter(
        transform = |interval:Value| ToCheckRequire::require_check(interval)
    ))]
    pub interval_by_time_range: NoCheck<Value>,
}

#[derive(Debug, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct FetcherCount(i8);

impl FetcherCount {
    pub fn new(count: i8) -> Self {
        // enum 从 0 开始， 数据库记录里面从1 开始
        // 从外部进入 +1
        Self(count + 1)
    }

    pub fn take(self) -> i8 { self.0 }
}

impl IntoActiveModel<model_config::ActiveModel> for FetcherConfig {
    fn into_active_model(self) -> model_config::ActiveModel {
        let mut active = model_config::ActiveModel {
            live_number: Set(self.live_number),
            fetcher_count: Set(self.fetcher_count.take()),
            group_name: Set(self.group_name),
            platform: Set(self.platform),
            datasource_id: Set(self.datasource_id),
            interval: Set(self.interval),
            ..Default::default()
        };

        let interval_time_range = match self.interval_by_time_range {
            Value::Null => None,
            value => Some(value.to_string()),
        };

        active.interval_by_time_range = Set(interval_time_range);

        active
    }
}

/// 用于验证FetcherConfig数组
pub type FetcherConfigVecChecker = IntoIterChecker<
    Vec<FetcherConfigUncheck>,
    FetcherConfigChecker,
    Vec<FetcherConfig>,
>;
