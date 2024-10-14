use checker::prefabs::{
    no_check::NoCheck, str_len_checker::StrMaxCharLenChecker,
    url_checker::UrlChecker, version_checker::VersionChecker,
};
use sea_orm::{IntoActiveModel, Set};
use sql_connection::ext_traits::ActiveModelUpdater;
use typed_builder::TypedBuilder;
use url::Url;

use super::CheckError;
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
    pub apk: Url,
    pub spare_apk: Url,
    pub baidu: Url,
    pub baidu_text: String,
}

#[checker::check_gen(
    uncheck = CeobeOperationAppVersionUncheck,
    checked = CeobeOperationAppVersion,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct CeobeOperationAppVersionChecker {
    pub version: VersionChecker<String>,
    pub force: NoCheck<bool>,
    pub last_force_version: VersionChecker<String>,
    pub description: StrMaxCharLenChecker<String, 4096>,
    pub apk: UrlChecker,
    pub spare_apk: UrlChecker,
    pub baidu: UrlChecker,
    pub baidu_text: StrMaxCharLenChecker<String, 32>,
}

impl IntoActiveModel<ActiveModel> for CeobeOperationAppVersion {
    fn into_active_model(self) -> ActiveModel {
        let CeobeOperationAppVersion {
            version,
            force,
            last_force_version,
            description,
            apk,
            spare_apk,
            baidu,
            baidu_text,
        } = self;
        let now = get_now_naive_date_time();
        ActiveModel {
            version: Set(version),
            force: Set(force),
            last_force_version: Set(last_force_version),
            description: Set(description),
            apk: Set(apk.to_string()),
            spare_apk: Set(spare_apk.to_string()),
            baidu: Set(baidu.to_string()),
            baidu_text: Set(baidu_text),
            create_at: Set(now),
            modify_at: Set(now),
            ..Default::default()
        }
    }
}

impl ActiveModelUpdater<ActiveModel> for CeobeOperationAppVersion {
    fn update_active(self, active_model: &mut ActiveModel) {
        let Self {
            version,
            force,
            last_force_version,
            description,
            apk,
            spare_apk,
            baidu,
            baidu_text,
        } = self;
        active_model.version = Set(version);
        active_model.force = Set(force);
        active_model.last_force_version = Set(last_force_version);
        active_model.description = Set(description);
        active_model.apk = Set(apk.to_string());
        active_model.spare_apk = Set(spare_apk.to_string());
        active_model.baidu = Set(baidu.to_string());
        active_model.baidu_text = Set(baidu_text);
        active_model.now_modify();
    }
}
