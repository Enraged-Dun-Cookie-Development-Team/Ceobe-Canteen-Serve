use checker::check_obj;
use range_limit::limits::max_limit::MaxRangeLimit;

use super::{
    super::models::PluginVersionChecked, version_checker::VersionChecker, download_resource_checker::DownloadResourceChecker, CheckError,
};

check_obj! {
    pub struct PluginVersionUncheck = PluginVersionChecker > PluginVersionChecked{
        version: VersionChecker,
        logo: MaxRangeLimit<String, 128>,
        title: MaxRangeLimit<String, 128>,
        description: MaxRangeLimit<String, 1024>,
        down: DownloadResourceChecker
    }
    err: CheckError
}
