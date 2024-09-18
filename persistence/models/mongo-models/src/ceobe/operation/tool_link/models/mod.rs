use mongodb::bson;
use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct LocalizedLanguage {
    #[serde(rename = "zh_CN")]
    pub zh_cn: String,
    #[serde(rename = "en_US")]
    pub en_us: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct LocalizedTags {
    #[serde(rename = "zh_CN")]
    pub zh_cn: Vec<String>,
    #[serde(rename = "en_US")]
    pub en_us: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all_fields = "SCREAMING_SNAKE_CASE")]
pub enum Region {
    ChinaMainland,
    ExceptChinaMainland,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Link {
    #[serde(default)]
    pub primary: bool,
    pub regionality: Region,
    pub localized_name: LocalizedLanguage,
    pub url: Url,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(all(
    name = "ToolLinkUpdate",
    extra(derive(Debug, Clone, Serialize, Deserialize))
))]
pub struct ToolLink {
    #[sub_model(ignore("ToolLinkUpdate"))]
    pub id: bson::Uuid,
    pub localized_name: LocalizedLanguage,
    pub localized_description: LocalizedLanguage,
    pub localized_slogan: LocalizedLanguage,
    pub localized_tags: LocalizedTags,
    pub icon_url: Url,
    pub links: Vec<Link>,
}

pub type ToolLinkCreate = ToolLinkUpdate;

impl From<ToolLinkUpdate> for ToolLink {
    fn from(
        ToolLinkUpdate {
            localized_name,
            localized_description,
            localized_slogan,
            localized_tags,
            icon_url,
            links,
        }: ToolLinkUpdate,
    ) -> Self {
        ToolLink {
            id: bson::Uuid::new(),
            localized_name,
            localized_description,
            localized_slogan,
            localized_tags,
            icon_url,
            links,
        }
    }
}
