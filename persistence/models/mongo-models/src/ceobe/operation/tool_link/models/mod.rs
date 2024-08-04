use mongodb::bson;
use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

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

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Link {
    pub primary: bool,
    pub regionality: String,
    pub service: String,
    pub localized_name: LocalizedLanguage,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct ToolLink {
    pub id: bson::Uuid,
    pub localized_name: LocalizedLanguage,
    pub localized_description: LocalizedLanguage,
    pub localized_slogen: LocalizedLanguage,
    pub localized_tags: LocalizedTags,
    pub icon_url: String,
    pub links: Vec<Link>,
}
