use checker::prefabs::{
    no_check::NoCheck, str_len_checker::StrMaxCharLenChecker,
    url_checker::UrlChecker, version_checker::VersionChecker,
};
use sea_orm::{IntoActiveModel, Set};
use typed_builder::TypedBuilder;
use url::Url;

use super::{ CheckError};
use crate::{
    ceobe_operation::desktop_version::models::model_desktop_version::ActiveModel,
    get_now_naive_date_time,
};

#[derive(Debug, TypedBuilder)]
pub struct CeobeOperationDesktopVersion {
    pub version: String,
    pub force: bool,
    pub last_force_version: String,
    pub description: String,
    pub exe: Url,
    pub spare_exe: Url,
    pub dmg: Url,
    pub spare_dmg: Url,
    pub baidu: Url,
    pub baidu_text: String,
}

#[checker::check_gen(
    uncheck = CeobeOperationDesktopVersionUncheck,
    checked = CeobeOperationDesktopVersion,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct CeobeOperationDesktopVersionChecker {
    pub version: VersionChecker<String>,
    pub force: NoCheck<bool>,
    pub last_force_version: VersionChecker<String>,
    pub description: StrMaxCharLenChecker<String, 4096>,
    pub exe: UrlChecker,
    pub spare_exe: UrlChecker,
    pub dmg: UrlChecker,
    pub spare_dmg: UrlChecker,
    pub baidu: UrlChecker,
    pub baidu_text: StrMaxCharLenChecker<String, 32>,
}

impl IntoActiveModel<ActiveModel> for CeobeOperationDesktopVersion {
    fn into_active_model(self) -> ActiveModel {
        let CeobeOperationDesktopVersion {
            version,
            force,
            last_force_version,
            description,
            exe,
            spare_exe,
            dmg,
            spare_dmg,
            baidu,
            baidu_text,
        } = self;
        let now = get_now_naive_date_time();
        ActiveModel {
            version: Set(version),
            force: Set(force),
            last_force_version: Set(last_force_version),
            description: Set(description),
            exe: Set(exe.to_string()),
            spare_exe: Set(spare_exe.to_string()),
            dmg: Set(dmg.to_string()),
            spare_dmg: Set(spare_dmg.to_string()),
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
    pub fn update_desktop_version(
        &mut self,
        CeobeOperationDesktopVersion {
            version,
            force,
            last_force_version,
            description,
            exe,
            spare_exe,
            dmg,
            spare_dmg,
            baidu,
            baidu_text,
        }: CeobeOperationDesktopVersion,
    ) {
        self.version = Set(version);
        self.force = Set(force);
        self.last_force_version = Set(last_force_version);
        self.description = Set(description);
        self.exe = Set(exe.to_string());
        self.spare_exe = Set(spare_exe.to_string());
        self.dmg = Set(dmg.to_string());
        self.spare_dmg = Set(spare_dmg.to_string());
        self.baidu = Set(baidu.to_string());
        self.baidu_text = Set(baidu_text);
        self.now_modify();
    }
}
