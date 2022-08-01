
use checker::{check_obj, prefabs::no_check::NoCheck};
use range_limit::limits::max_limit::MaxRangeLimit;
use typed_builder::TypedBuilder;

use super::CheckError;
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
        pub version: MaxRangeLimit<String, 10>,
        pub forcus: NoCheck<bool>,
        pub last_force_version: MaxRangeLimit<String, 10>,
        pub description: MaxRangeLimit<String, 2048>
    }
    err: CheckError
}

impl model_app_version::ActiveModel {
    
}
