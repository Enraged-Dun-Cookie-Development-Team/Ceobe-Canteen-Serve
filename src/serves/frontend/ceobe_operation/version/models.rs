use axum_prehandle::prefabs::query::QueryParams;
use checker::prefabs::option_checker::OptionChecker;
use mongo_migration::mongo_models::ceobe_operation::plugin_version::check::version_checker::VersionChecker as PluginVersionChecker;
use mongo_migration::mongo_models::ceobe_operation::plugin_version::models::Version;
use orm_migrate::sql_models::ceobe_operation::app_version::checkers::app_version_checker::{AppVersionChecker};
use serde::{Serialize, Deserialize};
use typed_builder::TypedBuilder;
use crate::models::sql::app_version::checkers::CheckError as AppCheckError;
use crate::models::mongo::plugin_version::check::CheckError as PluginCheckError;
use crate::utils::data_checker::PreLiteChecker;

use super::error::CeobeOperationVersionError;


// 用于app版本请求参数
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AppVersion {
    pub version: Option<String>
}
#[checker::check_gen(
    uncheck = AppVersionUncheck,
    checked = AppVersion,
    error = AppCheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct OptionAppVersionChecker {
    pub version: OptionChecker<AppVersionChecker>,
}

pub type OptionAppVersionCheckerPretreat =
    PreLiteChecker<QueryParams<AppVersionUncheck>, OptionAppVersionChecker, CeobeOperationVersionError>;


// 用于插件版本请求参数
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PluginVersion {
    pub version: Option<Version>
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
    PreLiteChecker<QueryParams<PluginVersionUncheck>, OptionPluginVersionChecker, CeobeOperationVersionError>;
