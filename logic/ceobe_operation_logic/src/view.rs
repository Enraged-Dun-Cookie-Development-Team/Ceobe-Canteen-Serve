use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DeleteOneToolLinkReq {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ToolLinkBackend {
    pub id: Option<i32>,
    pub nickname: String,
    pub avatar: String,
    pub jump_url: String,
    pub slogen: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ToolLinkFront {
    pub nickname: String,
    pub avatar: String,
    pub jump_url: String,
    pub slogen: String,
    pub description: String,
    pub tags: Vec<String>,
}
