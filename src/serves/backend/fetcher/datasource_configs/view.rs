use ceobe_qiniu_upload::{QiniuManager, ResponsePayload};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AvatarId {
    url: String,
}

impl AvatarId {
    pub(super) fn from_resp(
        ResponsePayload { key, .. }: ResponsePayload,
        qiniu: &QiniuManager,
    ) -> Self {
        Self {
            url: qiniu.concat_url(key),
        }
    }
}