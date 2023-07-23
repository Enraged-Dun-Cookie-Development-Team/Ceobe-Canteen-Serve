use checker::prefabs::{
    no_check::NoCheck, str_len_checker::StrMaxCharLenChecker,
};
use sea_orm::{IntoActiveModel, Set};
use typed_builder::TypedBuilder;

use super::{app_version_checker::AppVersionChecker, CheckError};
use crate::{
    ceobe_operation::app_version::models::model_app_version::ActiveModel,
    get_now_naive_date_time,
};

#[derive(Debug, TypedBuilder)]
pub struct CeobeOperationAppVersion {
    pub version: String,
    pub force: bool,
    pub last_force_version: String,
    pub description: String,
}

#[checker::check_gen(
    uncheck = CeobeOperationAppVersionUncheck,
    checked = CeobeOperationAppVersion,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct CeobeOperationAppVersionChecker {
    pub version: AppVersionChecker,
    pub force: NoCheck<bool>,
    pub last_force_version: AppVersionChecker,
    pub description: StrMaxCharLenChecker<String, 4096>,
}

impl IntoActiveModel<ActiveModel> for CeobeOperationAppVersion {
    fn into_active_model(self) -> ActiveModel {
        let CeobeOperationAppVersion {
            version,
            force,
            last_force_version,
            description,
        } = self;
        let now = get_now_naive_date_time();
        ActiveModel {
            version: Set(version),
            force: Set(force),
            last_force_version: Set(last_force_version),
            description: Set(description),
            create_at: Set(now),
            modify_at: Set(now),
            ..Default::default()
        }
    }
}

impl ActiveModel {
    #[allow(dead_code)]
    pub fn update_app_version(
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
