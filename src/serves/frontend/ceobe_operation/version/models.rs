use checker::{prefabs::option_checker::OptionChecker, QueryCheckExtract};
use mongo_migration::mongo_models::ceobe_operation::plugin_version::{
    check::{
        version_checker::VersionChecker as PluginVersionChecker,
        CheckError as PluginCheckError,
    },
    models::Version,
};
use orm_migrate::sql_models::ceobe_operation::app_version;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::error::CeobeOperationVersionError;

/// 用于app版本请求参数
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AppVersion {
    pub version: Option<String>,
}
#[checker::check_gen(
    uncheck = AppVersionUncheck,
    checked = AppVersion,
    error = app_version::CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct OptionAppVersionChecker {
    pub version: OptionChecker<app_version::AppVersionChecker>,
}

pub type OptionAppVersionCheckerPretreat =
    QueryCheckExtract<OptionAppVersionChecker, CeobeOperationVersionError>;

/// 用于插件版本请求参数
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PluginVersion {
    pub version: Option<Version>,
}
#[checker::check_gen(
    uncheck = PluginVersionUncheck,
    checked = PluginVersion,
    error = PluginCheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct OptionPluginVersionChecker {
    pub version: OptionChecker<PluginVersionChecker>,
}

pub type OptionPluginVersionCheckerPretreat =
    QueryCheckExtract<OptionPluginVersionChecker, CeobeOperationVersionError>;
