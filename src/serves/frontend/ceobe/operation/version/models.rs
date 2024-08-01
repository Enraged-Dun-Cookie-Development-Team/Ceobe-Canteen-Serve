use std::fmt::{Display, Formatter};

use checker::{
    prefabs::{
        option_checker::OptionChecker,
        version_checker::{Version, VersionChecker},
    },
    QueryCheckExtract,
};
use persistence::ceobe_operate::{
    desktop_version,
    models::{app_version, version::models::ReleasePlatform},
    plugin_version::{self, version},
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
    pub version: OptionChecker<VersionChecker<String>>,
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
    pub version: OptionChecker<version::Checker<Version>>,
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
    pub version: OptionChecker<VersionChecker<String>>,
}

pub type OptionDesktopVersionCheckerPretreat = QueryCheckExtract<
    OptionDesktopVersionChecker,
    CeobeOperationVersionError,
>;

#[derive(Deserialize, Clone, Debug)]
pub struct QueryReleaseVersion {
    pub version: Option<semver::Version>,
    pub platform: ReleasePlatform,
}

impl Display for QueryReleaseVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.version {
            Some(ver) => write!(f, "{}:{}", self.platform, ver),
            None => write!(f, "{}", self.platform),
        }
    }
}
