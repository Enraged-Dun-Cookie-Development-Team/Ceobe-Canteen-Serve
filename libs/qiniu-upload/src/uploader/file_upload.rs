use qiniu_upload_manager::AutoUploaderObjectParams;
use tracing::{instrument, info};

use super::ResponsePayload;
use crate::{error, FilePayload, PayloadLocal, Uploader};

impl Uploader {
    #[instrument(skip_all, fields(
        filename = ?payload.file_path().as_ref(),
        qiniu.bucket = payload.bucket(),
        qiniu.obj = payload.obj_name(),
        qiniu.file = payload.file_name()
    ))]
    pub async fn upload_file(
        &self, payload: impl PayloadLocal + FilePayload,
    ) -> Result<ResponsePayload, error::Error> {
        let auto_uploader = self
            .managers
            .get(payload.bucket())
            .ok_or_else(|| {
                error::Error::BucketNotInManage(payload.bucket().into())
            })?
            .get_default_upload();

        let param = AutoUploaderObjectParams::builder()
            .object_name(payload.obj_name())
            .file_name(payload.file_name())
            .content_type(payload.content_type())
            .build();

        let response = auto_uploader
            .async_upload_path(payload.file_path(), param)
            .await
            .map_err(error::Error::from)
            .and_then(|response| {
                serde_json::from_value::<ResponsePayload>(response)
                    .map_err(From::from)
            })?;
        info!(
            qiniu.response.hash = response.hash,
            qiniu.response.key = response.key
        );
        Ok(response)
    }
}
