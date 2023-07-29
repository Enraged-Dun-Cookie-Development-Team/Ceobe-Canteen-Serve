use checker::prefabs::{
    option_checker::OptionChecker, url_checker::UrlChecker,
};
use serde::Deserialize;

use super::{
    spare_link_checker::SpareLinkChecker, CheckError, DownloadResource,
};

#[checker::check_gen(
    uncheck = DownloadResourceUnchecked,
    checked = DownloadResource,
    error = CheckError
)]
#[derive(Debug, Deserialize)]
pub struct DownloadResourceChecker {
    crx: UrlChecker,
    spare_crx: OptionChecker<UrlChecker>,
    zip: UrlChecker,
    spare_zip: OptionChecker<UrlChecker>,
    chrome: UrlChecker,
    edge: UrlChecker,
    firefox: UrlChecker,
    spare: SpareLinkChecker,
}
