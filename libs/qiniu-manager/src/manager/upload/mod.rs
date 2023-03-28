use futures::Future;
use qiniu_upload_manager::AutoUploaderObjectParams;
use tracing::info;

use self::payload::{PayloadContent, PayloadLocal};
use crate::{error, ManagedUploader, Manager};

pub mod payload;
pub mod upload_field;
pub mod upload_file;
pub mod upload_json;

impl Manager {
    pub async fn custom_upload<'l, F, Fut, O>(
        &self, handle: F,
    ) -> Result<O, error::Error>
    where
        F: for<'r> FnOnce(&'r ManagedUploader) -> Fut,
        Fut: Future<Output = Result<O, error::Error>> + 'l,
    {
        let manager = &self.uploader;

        handle(manager).await
    }

    pub async fn upload(
        &self, payload: impl PayloadLocal + PayloadContent,
    ) -> Result<ResponsePayload, error::Error> {
        info!(
            content_type = %payload.content_type(),
            qiniu.uploader.obj = payload.obj_name(),
            qiniu.uploader.file = payload.file_name(),
        );

        let auto_uploader = self.uploader.get_default_upload();

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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ResponsePayload {
    pub hash: String,
    pub key: String,
}
