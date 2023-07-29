use checker::prefabs::{
    no_check::NoCheck, str_len_checker::StrMaxCharLenChecker,
    url_checker::UrlChecker,
};
use sea_orm::{IntoActiveModel, Set};
use typed_builder::TypedBuilder;
use url::Url;

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
    pub version: AppVersionChecker,
    pub force: NoCheck<bool>,
    pub last_force_version: AppVersionChecker,
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

impl ActiveModel {
    #[allow(dead_code)]
    pub fn update_app_version(
        &mut self,
        CeobeOperationAppVersion {
            version,
            force,
            last_force_version,
            description,
            apk,
            spare_apk,
            baidu,
            baidu_text,
        }: CeobeOperationAppVersion,
    ) {
        self.version = Set(version);
        self.force = Set(force);
        self.last_force_version = Set(last_force_version);
        self.description = Set(description);
        self.apk = Set(apk.to_string());
        self.spare_apk = Set(spare_apk.to_string());
        self.baidu = Set(baidu.to_string());
        self.baidu_text = Set(baidu_text);
        self.now_modify();
    }
}
