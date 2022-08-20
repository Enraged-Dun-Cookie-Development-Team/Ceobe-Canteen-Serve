use range_limit::limits::max_limit::MaxRangeLimit;
use serde::Deserialize;

use super::{
    download_resource_checker::DownloadResourceChecker,
    version_checker::VersionChecker, CheckError, PluginVersionChecked,
};

#[checker::check_gen(
    uncheck = PluginVersionUncheck,
    checked = PluginVersionChecked,
    error = CheckError
)]
#[derive(Debug, Deserialize)]
pub struct PluginVersionChecker {
    version: VersionChecker,
    title: MaxRangeLimit<String, 128>,
    description: MaxRangeLimit<String, 1024>,
    down: DownloadResourceChecker,
}
