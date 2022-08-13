use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

use crate::models::{
    mongo::plugin_version::models::{
        DownloadResource, PluginVersion, SpareLink,
    },
    sql::app_version::models::model_app_version,
};

// app版本
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AppVersionView {
    pub version: String,
    pub force: bool,
    pub last_force_version: String,
    pub description: String,
}

// app版本转换
impl From<model_app_version::Model> for AppVersionView {
    fn from(
        model_app_version::Model {
            version,
            force,
            last_force_version,
            description,
            ..
        }: model_app_version::Model,
    ) -> Self {
        Self {
            version,
            force,
            last_force_version,
            description,
        }
    }
}

// 插件版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpareLinkView(pub Url, pub String);
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DownloadView {
    crx: Url,
    zip: Url,
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
            zip,
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
