use std::borrow::Cow;

use modify_cache::ModifyState;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;
use persistence::ceobe_operate::models::announcement;

use crate::utils::time_format::naive_date_time_format;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AnnouncementItem {
    pub start_time: String,
    pub over_time: String,
    pub html: String,
    pub notice: bool,
}

impl From<announcement::Model> for AnnouncementItem {
    fn from(
        announcement::Model {
            start_time,
            over_time,
            content,
            img_url,
            notice,
            ..
        }: announcement::Model,
    ) -> Self {
        let image = Url::parse(&img_url)
            .map(|url| url.to_string())
            .unwrap_or_else(|_| format!(r#"/assets/image/{img_url}.png"#));

        Self {
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
            html: format!(
                r#"<div class="online-area"><img class="online-title-img radius" src="{image}"/><div>{content}</div></div>"#,
            ),
            notice,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_url() {
        let url = url::Url::parse("icon");
        println!("{url:?}")
    }
}

/// 用于请求头缓存信息生成
pub struct AnnouncementItems(pub(super) Vec<AnnouncementItem>);
impl AnnouncementItems {
    pub(super) fn into_inner(
        this: Option<Self>,
    ) -> Option<Vec<AnnouncementItem>> {
        this.map(|v| v.0)
    }
}
impl ModifyState for AnnouncementItems {
    type Identify = Vec<AnnouncementItem>;

    fn get_identify(&self) -> Cow<'_, Self::Identify> {
        Cow::Borrowed(&self.0)
    }
}
