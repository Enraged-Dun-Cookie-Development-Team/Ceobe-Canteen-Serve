use checker::{check_obj, prefabs::url_checker::UrlChecker};
use serde::Deserialize;

use super::{spare_link_checker::SpareLinkChecker, CheckError};
use crate::addon_version::models::DownloadResource;

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
