
use checker::{check_obj, prefabs::no_check::NoCheck};
use range_limit::limits::max_limit::MaxRangeLimit;
use typed_builder::TypedBuilder;

use super::{CheckError, app_version_checker::AppVersionChecker};
use crate::ceobe_operation::app_version::models::model_app_version;

#[derive(Debug, TypedBuilder)]
pub struct CeobeOperationAppVersion {
    pub version: String,
    pub forcus: bool,
    pub last_force_version: String,
    pub description: String,
}

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct CeobeOperationAppVersionUncheck = CeobeOperationAppVersionChecker > CeobeOperationAppVersion{
        pub version: AppVersionChecker,
        pub forcus: NoCheck<bool>,
        pub last_force_version: AppVersionChecker,
        pub description: MaxRangeLimit<String, 2048>
    }
    err: CheckError
}

impl model_app_version::ActiveModel {
    
}
