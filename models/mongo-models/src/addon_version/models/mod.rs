use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;
use url::Url;

use crate::RecordUnit;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    name = "PluginVersionChecked",
    extra(derive(Debug, TypedBuilder))
))]
pub struct AddonVersion {
    version: Version,
    #[sub_model(ignore("PluginVersionChecked"))]
    time_record: RecordUnit,
    logo: String,
    title: String,
    description: String,
    down: DownloadResource,
}
