
use checker::{check_obj, prefabs::no_check::NoCheck};
use range_limit::limits::max_limit::MaxRangeLimit;
use sea_orm::Set;
use typed_builder::TypedBuilder;

use super::{CheckError, app_version_checker::AppVersionChecker};
use crate::{ceobe_operation::app_version::models::model_app_version, get_now_naive_date_time};

#[derive(Debug, TypedBuilder)]
pub struct CeobeOperationAppVersion {
    pub version: String,
    pub force: bool,
    pub last_force_version: String,
    pub description: String,
}

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct CeobeOperationAppVersionUncheck = CeobeOperationAppVersionChecker > CeobeOperationAppVersion{
        pub version: AppVersionChecker,
        pub force: NoCheck<bool>,
        pub last_force_version: AppVersionChecker,
        pub description: MaxRangeLimit<String, 4096>
    }
    err: CheckError
}

impl model_app_version::ActiveModel {
    // 新建app更新信息
    pub(in crate::ceobe_operation::app_version) fn create_app_version(
        CeobeOperationAppVersion {
            version,
            force,
            last_force_version,
            description,
        }: CeobeOperationAppVersion,
    ) -> Self {
        let now = get_now_naive_date_time();
        Self {
            version: Set(version),
            force: Set(force), 
            last_force_version: Set(last_force_version),
            description: Set(description),
            create_at: Set(now),
            modify_at: Set(now),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub(in crate::ceobe_operation::app_version) fn update_app_version(
        &mut self,
        CeobeOperationAppVersion {
            version,
            force,
            last_force_version,
            description,
        }: CeobeOperationAppVersion,
    ) {
        self.version = Set(version);
        self.force = Set(force);
        self.last_force_version = Set(last_force_version);
        self.description = Set(description);
        self.now_modify();
    }
}
