use qiniu_cdn_upload::{
    update_payload::UploadPayload, update_source::FieldSource,
};
use uuid::Uuid;

pub mod controllers;
pub mod error;

pub struct ToolAvatarPayload(String);

impl ToolAvatarPayload {
    pub fn new() -> Self { Self(Uuid::new_v4().to_string()) }
}

impl UploadPayload for ToolAvatarPayload {
    type Source = FieldSource;

    const DIR: &'static str = "images/tool-avatar";

    fn obj_name(&self) -> &str { &self.0 }
}
