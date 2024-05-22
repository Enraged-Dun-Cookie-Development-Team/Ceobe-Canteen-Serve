use persistence::ceobe_operate::models::tool_link::{
    self, models::model_tool_link::FrontendToolLink,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

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
