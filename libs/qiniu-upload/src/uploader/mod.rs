mod builder;
mod payload;
mod upload_field;
mod upload_file;
mod upload_json;
use std::{collections::HashMap, fmt::Debug};

use futures::Future;
use qiniu_upload_manager::AutoUploaderObjectParams;
use smallstr::SmallString;
use tracing::info;
pub use upload_json::JsonPayload;

pub use self::{
    builder::{ManagedUploader, UploaderBuilder},
    payload::{ByteUploader, FilePayload, PayloadContent, PayloadLocal},
};
use crate::{error, SecretConfig};
#[derive(Debug)]
pub struct Uploader {
    pub(crate) managers:
        HashMap<SmallString<[u8; 64]>, ManagedUploader, ahash::RandomState>,
}

impl Uploader {
    pub fn builder(secret: &impl SecretConfig) -> builder::UploaderBuilder {
        UploaderBuilder::new(secret)
    }

    pub async fn custom_upload<'l, L, F, Fut, O>(
        &self, local: &'l L, handle: F,
    ) -> Result<O, error::Error>
    where
        L: PayloadLocal,
        F: for<'r> FnOnce(&'r ManagedUploader) -> Fut,
        Fut: Future<Output = Result<O, error::Error>> + 'l,
    {
        let manager = self.managers.get(local.bucket()).ok_or_else(|| {
            error::Error::BucketNotInManage(local.bucket().into())
        })?;

        handle(manager).await
    }

    pub async fn upload(
        &self, payload: impl PayloadLocal + PayloadContent,
    ) -> Result<ResponsePayload, error::Error> {
        info!(
            content_type = %payload.content_type(),
            qiniu.uploader.bucket = payload.bucket(),
            qiniu.uploader.obj = payload.obj_name(),
            qiniu.uploader.file = payload.file_name(),
        );

        let auto_uploader = self
            .managers
            .get(payload.bucket())
            .ok_or_else(|| {
                error::Error::BucketNotInManage(payload.bucket().into())
            })?
            .get_default_upload();

        let params = AutoUploaderObjectParams::builder()
            .object_name(payload.obj_name())
            .file_name(payload.file_name())
            .content_type(payload.content_type())
            .build();

        let response = auto_uploader
            .async_upload_reader(Box::pin(payload.payload()?), params)
            .await?;
        let response = serde_json::from_value::<ResponsePayload>(response)?;

        info!(
            qiniu.response.hash = response.hash,
            qiniu.response.key = response.key
        );

        Ok(response)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ResponsePayload {
    pub hash: String,
    pub key: String,
}
