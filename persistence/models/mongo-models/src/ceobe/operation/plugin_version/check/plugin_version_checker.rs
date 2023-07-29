use checker::prefabs::version_checker::{VersionChecker, Version};
use range_limit::limits::max_limit::MaxRangeLimit;
use serde::Deserialize;

use super::{
    download_resource_checker::DownloadResourceChecker,
    CheckError, PluginVersionChecked,
};

#[checker::check_gen(
    uncheck = PluginVersionUncheck,
    checked = PluginVersionChecked,
    error = CheckError
)]
#[derive(Debug, Deserialize)]
pub struct PluginVersionChecker {
    version: VersionChecker<Version>,
    title: MaxRangeLimit<String, 128>,
    description: MaxRangeLimit<String, 1024>,
    down: DownloadResourceChecker,
}
