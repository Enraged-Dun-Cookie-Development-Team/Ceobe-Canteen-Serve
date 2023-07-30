use persistence::ceobe_operate::{
    desktop_version,
    models::app_version,
    plugin_version::{DownloadResource, PluginVersion, SpareLink},
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

/// app版本
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AppVersionView {
    pub version: String,
    pub force: bool,
    pub last_force_version: String,
    pub description: String,
    pub apk: String,
    pub spare_apk: String,
    pub baidu: String,
    pub baidu_text: String,
}

/// app版本转换
impl From<app_version::Model> for AppVersionView {
    fn from(
        app_version::Model {
            version,
            force,
            last_force_version,
            description,
            apk,
            spare_apk,
            baidu,
            baidu_text,
            ..
        }: app_version::Model,
    ) -> Self {
        Self {
            version,
            force,
            last_force_version,
            description,
            apk,
            spare_apk,
            baidu,
            baidu_text,
        }
    }
}

/// app版本
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DesktopVersionView {
    pub version: String,
    pub force: bool,
    pub last_force_version: String,
    pub description: String,
    pub exe: String,
    pub spare_exe: String,
    pub dmg: String,
    pub spare_dmg: String,
    pub baidu: String,
    pub baidu_text: String,
}

/// 桌面端版本转换
impl From<desktop_version::Model> for DesktopVersionView {
    fn from(
        desktop_version::Model {
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
            ..
        }: desktop_version::Model,
    ) -> Self {
        Self {
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
        }
    }
}

/// 插件版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpareLinkView(pub Url, pub String);
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DownloadView {
    crx: Url,
    spare_crx: Option<Url>,
    zip: Url,
    spare_zip: Option<Url>,
    chrome: Url,
    edge: Url,
    firefox: Url,
    spare: SpareLinkView,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PluginVersionView {
    pub version: String,
    pub description: String,
    pub title: String,
    pub down: DownloadView,
}

impl From<SpareLink> for SpareLinkView {
    fn from(SpareLink { url, msg }: SpareLink) -> Self { Self(url, msg) }
}

impl From<DownloadResource> for DownloadView {
    fn from(
        DownloadResource {
            crx,
            spare_crx,
            zip,
            spare_zip,
            chrome,
            edge,
            firefox,
            spare,
        }: DownloadResource,
    ) -> Self {
        Self {
            crx,
            zip,
            chrome,
            edge,
            firefox,
            spare: spare.into(),
            spare_crx,
            spare_zip,
        }
    }
}

impl From<PluginVersion> for PluginVersionView {
    fn from(
        PluginVersion {
            version,
            description,
            title,
            down,
            ..
        }: PluginVersion,
    ) -> Self {
        Self {
            version: version.to_string(),
            description,
            title,
            down: down.into(),
        }
    }
}
