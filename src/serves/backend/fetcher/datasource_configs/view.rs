use ceobe_qiniu_upload::{QiniuUploader, ResponsePayload};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AvatarId {
    url: String,
}

impl AvatarId {
    pub(super) fn from_resp(
        ResponsePayload { key, .. }: ResponsePayload,
        uploader: &QiniuUploader,
    ) -> Self {
        Self {
            url: uploader.concat_url(key),
        }
    }
}