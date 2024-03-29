use tracing::instrument;

use super::{
    payload::{ByteUploader, PayloadLocal},
    ResponsePayload,
};
use crate::{error, Manager};

impl Manager {
    #[instrument(skip_all, fields(
        content_type = field.content_type(),
        qiniu.obj = local.obj_name(),
        qiniu.file = local.file_name()
    ))]
    pub async fn upload_field<'a>(
        &self, field: axum::extract::multipart::Field<'a>,
        local: impl PayloadLocal,
    ) -> Result<ResponsePayload, error::Error> {
        let payload = ByteUploader::from_field(field, local).await?;

        self.upload(payload).await
    }
}
