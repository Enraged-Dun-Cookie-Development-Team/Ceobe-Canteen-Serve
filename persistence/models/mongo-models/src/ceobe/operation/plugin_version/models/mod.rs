use std::borrow::Cow;

use checker::prefabs::version_checker::Version;
use modify_cache::ModifyState;
use serde::{Deserialize, Serialize};
pub use spare_link::SpareLink;
use sub_model::SubModel;
use typed_builder::TypedBuilder;
use url::Url;

use crate::{RecordUnit, RecordUnitUpdater, SetRecordUnit};

pub mod spare_link;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]

pub struct DownloadResource {
    pub crx: Url,
    pub spare_crx: Option<Url>,
    pub zip: Url,
    pub spare_zip: Option<Url>,
    pub chrome: Url,
    pub edge: Url,
    pub firefox: Url,
    pub spare: SpareLink,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(all(
    vis = "pub",
    name = "PluginVersionChecked",
    extra(derive(Debug, TypedBuilder, Clone))
))]
pub struct PluginVersion {
    pub version: Version,
    #[sub_model(ignore("PluginVersionChecked"))]
    pub time_record: RecordUnit,
    pub title: String,
    pub description: String,
    pub down: DownloadResource,
}

impl ModifyState for PluginVersion {
    type Identify = Self;

    fn get_last_modify_time(&self) -> Option<Cow<'_, chrono::NaiveDateTime>> {
        Some(Cow::Owned(
            self.time_record.modify_at.to_chrono().naive_local(),
        ))
    }

    fn get_identify(&self) -> Cow<'_, Self::Identify> { Cow::Borrowed(self) }
}

impl RecordUnitUpdater for PluginVersion {
    type Source = PluginVersionChecked;

    fn get_mut(&mut self) -> &mut RecordUnit { &mut self.time_record }
}

impl SetRecordUnit for PluginVersionChecked {
    type Target = PluginVersion;

    fn into_with_time_record(self, time_record: RecordUnit) -> Self::Target {
        let Self {
            version,
            title,
            description,
            down,
        } = self;

        Self::Target {
            version,
            time_record,
            title,
            description,
            down,
        }
    }
}
