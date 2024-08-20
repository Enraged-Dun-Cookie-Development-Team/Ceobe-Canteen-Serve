use checker::prefabs::{
    collect_checkers::iter_checkers::IntoIterChecker, no_check::NoCheck,
};
use mongodb::bson;
use serde::Deserialize;
use url::Url;

use super::CheckError;
use crate::ceobe::operation::tool_link::models::{
    Link, LocalizedLanguage, LocalizedTags, ToolLink,
};

#[checker::check_gen(
    uncheck = ToolLinkUnCheck,
    checked = ToolLink,
    error = CheckError
)]
#[derive(Debug, Deserialize)]
pub struct ToolLinkChecker {
    id: NoCheck<bson::Uuid>,
    localized_name: NoCheck<LocalizedLanguage>,
    localized_description: NoCheck<LocalizedLanguage>,
    localized_slogan: NoCheck<LocalizedLanguage>,
    localized_tags: NoCheck<LocalizedTags>,
    icon_url: NoCheck<Url>,
    links: IntoIterChecker<Vec<LinkUnCheck>, LinkChecker, Vec<Link>>,
}

#[checker::check_gen(
    uncheck = LinkUnCheck,
    checked = Link,
    error = CheckError
)]
#[derive(Debug, Deserialize)]
pub struct LinkChecker {
    primary: NoCheck<bool>,
    regionality: NoCheck<String>,
    service: NoCheck<String>,
    localized_name: NoCheck<LocalizedLanguage>,
    url: NoCheck<Url>,
}
