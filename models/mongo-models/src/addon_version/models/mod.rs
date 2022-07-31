use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

use crate::RecordUnit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version(u32, u32, u32, Option<String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpareLink(Url, String);

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DownloadResource {
    crx: Url,
    zip: Url,
    chrome: Url,
    edge: Url,
    firefox: Url,
    spare: SpareLink,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AddonVersion {
    version: Version,
    time_record: RecordUnit,
    logo: String,
    title: String,
    description: String,
    down: DownloadResource,
}
