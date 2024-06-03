use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;
use modify_cache::ModifyState;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct LocalizedLanguage {
    pub zh_CN: String,
    pub en_US: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct LocalizedTags {
    pub zh_CN: Vec<String>,
    pub en_US: Vec<String>,
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
#[sub_model(all(
    vis = "pub",
    name = "ToolLinkChecked",
    extra(derive(Debug, TypedBuilder))
))]
pub struct ToolLink {
    pub id: String,
    pub localized_name: LocalizedLanguage,
    pub localized_description: LocalizedLanguage,
    pub localized_slogen: LocalizedLanguage,
    pub localized_tags: LocalizedTags,
    pub icon_url: String,
    pub links: Vec<Link>,
}

impl ModifyState for ToolLink {
    type Identify = Self;

    fn get_identify(&self) -> Cow<'_, Self::Identify> { Cow::Borrowed(self) }
}