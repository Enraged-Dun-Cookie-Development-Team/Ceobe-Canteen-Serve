use checker::{prefabs::{option_checker::OptionChecker, version_checker::VersionChecker}, QueryCheckExtract};
use persistence::ceobe_operate::{
    models::app_version,
    plugin_version::{self, version},
    desktop_version,
};
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
    pub version: OptionChecker<VersionChecker>,
}

pub type OptionAppVersionCheckerPretreat =
    QueryCheckExtract<OptionAppVersionChecker, CeobeOperationVersionError>;

/// 用于插件版本请求参数
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PluginVersion {
    pub version: Option<version::Checked>,
}
#[checker::check_gen(
    uncheck = PluginVersionUncheck,
    checked = PluginVersion,
    error = plugin_version::CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct OptionPluginVersionChecker {
    pub version: OptionChecker<version::Checker>,
}

pub type OptionPluginVersionCheckerPretreat =
    QueryCheckExtract<OptionPluginVersionChecker, CeobeOperationVersionError>;

/// 用于桌面端版本请求参数
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DesktopVersion {
    pub version: Option<String>,
}
#[checker::check_gen(
    uncheck = DesktopVersionUncheck,
    checked = DesktopVersion,
    error = desktop_version::CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct OptionDesktopVersionChecker {
    pub version: OptionChecker<VersionChecker>,
}

pub type OptionDesktopVersionCheckerPretreat =
    QueryCheckExtract<OptionDesktopVersionChecker, CeobeOperationVersionError>;
