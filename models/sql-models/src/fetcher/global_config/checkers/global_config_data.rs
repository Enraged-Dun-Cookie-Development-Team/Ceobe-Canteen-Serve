
use checker::{check_obj, prefabs::collect_checkers::iter_checkers::IntoIterChecker};
use sea_orm::Set;
use typed_builder::TypedBuilder;
use range_limit::{limits::max_limit::MaxLimit, RangeBoundLimit};
use crate::fetcher::global_config::models::model_global_config;

use super::CheckError;

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;

#[derive(Debug, TypedBuilder)]
pub struct FetcherGlobalConfig {
    pub key: String,
    pub value: String
}

// 对上传数据进行校验
#[check_obj(
    uncheck = FetcherGlobalConfigUncheck,
    checked = FetcherGlobalConfig,
    error = CheckError
)]
#[derive(serde::Deserialize, Debug)]
pub struct FetcherGlobalConfigChecker {
    pub key: MaxLimitString<64>,
    pub value: MaxLimitString<64>,
}

impl model_global_config::ActiveModel {
    pub(in crate::fetcher::global_config) fn global_config_into_active_model(
        FetcherGlobalConfig {
            key,
            value,
        }: FetcherGlobalConfig,
    ) -> Self {
        Self {
            key: Set(key),
            value: Set(value),
            ..Default::default()
        }
    }
}

pub type FetcherGlobalConfigVecChecker = IntoIterChecker<Vec<FetcherGlobalConfigUncheck>, FetcherGlobalConfigChecker, Vec<FetcherGlobalConfig>>;
