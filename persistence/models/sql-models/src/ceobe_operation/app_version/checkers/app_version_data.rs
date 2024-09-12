use checker::prefabs::{
    no_check::NoCheck, str_len_checker::StrMaxCharLenChecker,
    url_checker::UrlChecker, version_checker::VersionChecker,
};
use sea_orm::{IntoActiveModel, Set};
use tracing_unwrap::ResultExt;
use typed_builder::TypedBuilder;
use url::Url;

use super::{super::models::model_app_version, CheckError};
use crate::{
    ceobe_operation::app_version::models::model_app_version::ActiveModel,
    get_now_naive_date_time,
};

#[derive(Debug, TypedBuilder, Clone)]
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

impl From<model_app_version::Model> for CeobeOperationAppVersion {
    fn from(
        model_app_version::Model {
            version,
            force,
            last_force_version,
            description,
            apk,
            spare_apk,
            baidu,
            baidu_text,
            ..
        }: model_app_version::Model,
    ) -> Self {
        Self {
            version,
            force,
            last_force_version,
            description,
            apk: apk.parse().unwrap_or_log(),
            spare_apk: spare_apk.parse().unwrap_or_log(),
            baidu: baidu.parse().unwrap_or_log(),
            baidu_text,
        }
    }
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
