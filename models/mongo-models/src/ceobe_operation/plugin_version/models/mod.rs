use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;
use url::Url;

use crate::RecordUnit;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Version(pub u32, pub u32, pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpareLink(pub Url, pub String);

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DownloadResource {
    crx: Url,
    zip: Url,
    chrome: Url,
    edge: Url,
    firefox: Url,
    spare: SpareLink,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(all(
    vis = "pub",
    name = "PluginVersionChecked",
    extra(derive(Debug, TypedBuilder))
))]
pub struct PluginVersion {
    pub version: Version,
    #[sub_model(ignore("PluginVersionChecked"))]
    pub time_record: RecordUnit,
    pub logo: String,
    pub title: String,
    pub description: String,
    pub down: DownloadResource,
}

impl PluginVersionChecked {
    pub fn into_with_time_record(
        self, time_record: RecordUnit,
    ) -> PluginVersion {
        let Self {
            version,
            logo,
            title,
            description,
            down,
        } = self;

        PluginVersion {
            version,
            time_record,
            logo,
            title,
            description,
            down,
        }
    }
}
