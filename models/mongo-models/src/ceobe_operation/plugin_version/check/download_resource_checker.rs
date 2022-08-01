use checker::{check_obj, prefabs::url_checker::UrlChecker};
use serde::Deserialize;

use super::{
    spare_link_checker::SpareLinkChecker, CheckError, DownloadResource,
};

check_obj! {
    #[derive(Debug, Deserialize)]
    pub struct DownloadResourceUnchecked = DownloadResourceChecker > DownloadResource{
        crx: UrlChecker,
        zip: UrlChecker,
        chrome: UrlChecker,
        edge: UrlChecker,
        firefox: UrlChecker,
        spare: SpareLinkChecker
    }
    err: CheckError
}
