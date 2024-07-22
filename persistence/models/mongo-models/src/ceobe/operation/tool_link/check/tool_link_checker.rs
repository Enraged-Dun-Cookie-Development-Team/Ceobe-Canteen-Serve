use checker::{
    prefabs::{
        collect_checkers::iter_checkers::IntoIterChecker, no_check::NoCheck,
    },
    Checker,
};
use futures::future::{ready, Ready};
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
    id: IdChecker,
    localized_name: NoCheck<LocalizedLanguage>,
    localized_description: NoCheck<LocalizedLanguage>,
    localized_slogen: NoCheck<LocalizedLanguage>,
    localized_tags: NoCheck<LocalizedTags>,
    icon_url: StringToUrlChecker,
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
    url: StringToUrlChecker,
}

pub struct IdChecker;

impl Checker for IdChecker {
    type Args = ();
    type Checked = bson::Uuid;
    type Err = CheckError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = Option<String>;

    fn check(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready(match uncheck {
            None => Ok(bson::Uuid::new()),
            Some(id) => bson::Uuid::parse_str(id).map_err(Into::into),
        })
    }
}

pub struct StringToUrlChecker;

impl Checker for StringToUrlChecker {
    type Args = ();
    type Checked = String;
    type Err = CheckError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready(Url::parse(&uncheck).map(|_v| uncheck).map_err(Into::into))
    }
}
