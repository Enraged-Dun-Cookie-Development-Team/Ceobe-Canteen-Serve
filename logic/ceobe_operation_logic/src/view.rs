use persistence::{
    ceobe_operate::{
        announcement,
        models::tool_link::{
            self, models::model_tool_link::FrontendToolLink,
        },
        resource::{
            self, all_available,
            countdown::{self, CountdownType},
        },
        tool_link_mongodb::models::{Link, ToolLink, ToolLinkUpdate},
        video,
    },
    help_crates::naive_date_time_format,
    mongodb::mongodb::bson,
};
use serde::{Deserialize, Serialize};
use tencent_cloud_server::cdn::purge_urls_cache::PurgeCachePath;
use typed_builder::TypedBuilder;
use url::Url;

use crate::error::LogicError;

#[derive(Debug, Clone, Deserialize, TypedBuilder)]
pub struct DeleteOneToolLinkReq {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct ToolLinkResp {
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub nickname: String,
    pub avatar: String,
    pub jump_url: String,
    pub slogan: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl TryInto<ToolLinkResp> for tool_link::Model {
    type Error = LogicError;

    fn try_into(self) -> Result<ToolLinkResp, Self::Error> {
        Ok(ToolLinkResp::builder()
            .id(Some(self.id))
            .avatar(self.avatar)
            .nickname(self.nickname)
            .jump_url(self.jump_url)
            .slogan(self.slogan)
            .description(self.description)
            .tags(serde_json::from_str::<Vec<String>>(&self.tags)?)
            .build())
    }
}

impl TryInto<ToolLinkResp> for FrontendToolLink {
    type Error = LogicError;

    fn try_into(self) -> Result<ToolLinkResp, Self::Error> {
        Ok(ToolLinkResp::builder()
            .avatar(self.avatar)
            .nickname(self.nickname)
            .jump_url(self.jump_url)
            .slogan(self.slogan)
            .description(self.description)
            .tags(serde_json::from_str::<Vec<String>>(&self.tags)?)
            .build())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AnnouncementFrontResp {
    pub start_time: String,
    pub over_time: String,
    pub html: String,
    pub notice: bool,
}

impl From<announcement::Model> for AnnouncementFrontResp {
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

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AnnouncementBackResp {
    pub start_time: String,
    pub over_time: String,
    pub content: String,
    pub img_url: String,
    pub notice: bool,
}

impl From<announcement::Model> for AnnouncementBackResp {
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
        Self {
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
            content,
            img_url,
            notice,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Resource {
    #[serde(rename = "resources")]
    resource_all_available: AllAvailable,
    countdown: Vec<Countdown>,
}

impl
    From<(
        resource::all_available::Model,
        Vec<resource::countdown::Model>,
    )> for Resource
{
    fn from(
        (raa, cd): (
            resource::all_available::Model,
            Vec<resource::countdown::Model>,
        ),
    ) -> Self {
        Self {
            resource_all_available: raa.into(),
            countdown: cd.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AllAvailable {
    start_time: String,
    over_time: String,
}
impl From<all_available::Model> for AllAvailable {
    fn from(
        all_available::Model {
            over_time,
            start_time,
            ..
        }: all_available::Model,
    ) -> Self {
        Self {
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Countdown {
    #[serde(rename = "text")]
    message: String,
    #[serde(rename = "remark")]
    banner_info: String,
    countdown_type: Option<CountdownType>,
    #[serde(rename = "time")]
    countdown_end: String,
    start_time: String,
    over_time: String,
}
impl From<countdown::Model> for Countdown {
    fn from(
        countdown::Model {
            start_time,
            message,
            countdown_end,
            banner_info,
            over_time,
            countdown_type,
            ..
        }: countdown::Model,
    ) -> Self {
        Self {
            message,
            banner_info,
            countdown_type,
            countdown_end: naive_date_time_format(countdown_end),
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct VideoItem {
    pub bv: String,
    pub start_time: String,
    pub over_time: String,
    pub title: String,
    pub author: String,
    pub video_link: String,
    #[serde(rename = "cover_img")]
    pub cover_image: String,
}

impl From<video::Model> for VideoItem {
    fn from(
        video::Model {
            bv,
            start_time,
            over_time,
            title,
            author,
            video_link,
            cover_image,
            ..
        }: video::Model,
    ) -> Self {
        Self {
            bv,
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
            title,
            author,
            video_link,
            cover_image,
        }
    }
}

pub(super) struct OperationTcCdnPath;

impl OperationTcCdnPath {
    /// 公告列表
    pub const ANNOUNCEMENT_LIST_PATH: PurgeCachePath =
        PurgeCachePath::new("/cdn/operate/announcement/list");
    /// 资源列表
    pub const RESOURCE_LIST_PATH: PurgeCachePath =
        PurgeCachePath::new("/cdn/operate/resource/get");
    /// 友联列表
    pub const TOOL_LINK_LIST: PurgeCachePath =
        PurgeCachePath::new("/cdn/operate/toolLink/list");
    /// 视频列表
    pub const VIDEO_LIST_PATH: PurgeCachePath =
        PurgeCachePath::new("/cdn/operate/video/list");
}

pub type ToolLinkCreateMongoReq = ToolLinkUpdate;

pub type ToolLinkUpdateMongoReq = ToolLink;

#[derive(Debug, Clone, Deserialize)]
pub struct ToolLinkDeleteMongoReq {
    pub id: bson::Uuid,
}

pub type LinkMongoReq = Link;
