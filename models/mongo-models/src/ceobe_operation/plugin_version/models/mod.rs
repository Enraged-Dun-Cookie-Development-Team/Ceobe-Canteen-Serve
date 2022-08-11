pub mod spare_link;
pub mod version;
use serde::{Deserialize, Serialize};
pub use spare_link::SpareLink;
use sub_model::SubModel;
use typed_builder::TypedBuilder;
use url::Url;
pub use version::Version;

use crate::RecordUnit;
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(all(
    name = "DownloadView",
    extra(derive(Deserialize, Serialize, Debug))
))]
pub struct DownloadResource {
    crx: Url,
    zip: Url,
    chrome: Url,
    edge: Url,
    firefox: Url,
    #[sub_model(having(
        for = "DownloadView",
        to_type(ty = "(String, String)", by = "SpareLink::into_tuple")
    ))]
    spare: SpareLink,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(
    all(
        vis = "pub",
        name = "PluginVersionChecked",
        extra(derive(Debug, TypedBuilder))
    ),
    all(
        name = "PluginVersionView",
        extra(derive(Deserialize, Serialize, Debug))
    )
)]
pub struct PluginVersion {
    #[sub_model(having(
        for = "PluginVersionView",
        to_type(ty = "String", by = "Version::to_version_str")
    ))]
    pub version: Version,
    #[sub_model(ignore("PluginVersionChecked"), ignore("PluginVersionView"))]
    pub time_record: RecordUnit,
    pub logo: String,
    pub title: String,
    pub description: String,
    #[sub_model(having(
        for = "PluginVersionView",
        to_type(ty = "DownloadView", by = "Into::into")
    ))]
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
