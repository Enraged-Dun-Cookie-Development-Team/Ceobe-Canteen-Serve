use checker::{
    check_obj, prefabs::{collect_checkers::iter_checkers::IntoIterChecker, str_len_checker::StrMaxCharLenChecker},
};
use sea_orm::Set;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::fetcher::global_config::models::model_global_config;

#[derive(Debug, TypedBuilder)]
pub struct FetcherGlobalConfig {
    pub key: String,
    pub value: String,
}

/// 对上传数据进行校验
#[check_obj(
    uncheck = FetcherGlobalConfigUncheck,
    checked = FetcherGlobalConfig,
    error = CheckError
)]
#[derive(serde::Deserialize, Debug)]
pub struct FetcherGlobalConfigChecker {
    pub key: StrMaxCharLenChecker<String, 64>,
    pub value: StrMaxCharLenChecker<String, 64>,
}

impl model_global_config::ActiveModel {
    pub(in crate::fetcher::global_config) fn global_config_into_active_model(
        FetcherGlobalConfig { key, value }: FetcherGlobalConfig,
    ) -> Self {
        Self {
            key: Set(key),
            value: Set(value),
            ..Default::default()
        }
    }
}

/// 用于验证FetcherGlobalConfig数组
pub type FetcherGlobalConfigVecChecker = IntoIterChecker<
    Vec<FetcherGlobalConfigUncheck>,
    FetcherGlobalConfigChecker,
    Vec<FetcherGlobalConfig>,
>;
