pub mod spare_link;
pub mod version;
use std::borrow::Cow;

use modify_cache::ModifyState;
use serde::{Deserialize, Serialize};
pub use spare_link::SpareLink;
use sub_model::SubModel;
use typed_builder::TypedBuilder;
use url::Url;
pub use version::Version;

use crate::RecordUnit;
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]

pub struct DownloadResource {
    pub crx: Url,
    pub zip: Url,
    pub chrome: Url,
    pub edge: Url,
    pub firefox: Url,
    pub spare: SpareLink,
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

impl ModifyState for PluginVersion {
    type Identify = Self;

    fn get_last_modify_time(&self) -> Option<Cow<'_, chrono::NaiveDateTime>> {
        Some(Cow::Owned(self.time_record.modify_at.to_chrono().naive_local()))
    }

    fn get_identify(&self) -> Cow<'_, Self::Identify> { Cow::Borrowed(self) }
}