use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Version(u32, u32, u32, Option<String>);

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DescriptionSegment {
    subtitle: Option<String>,
    detail: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DownloadResource {
    crx: String,
    zip: String,
    chrome: String,
    edge: String,
    firefox: String,
    spare: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AddonVersion {
    logo: String,
    version: Version,
    title: String,
    description: Vec<DescriptionSegment>,
    down: DownloadResource,
}
