use axum::extract::multipart::{Field, MultipartError};
use bytes::Bytes;
use futures::io::Cursor;
use mime::APPLICATION_OCTET_STREAM;
use smallstr::SmallString;
use tracing::instrument;

use super::ResponsePayload;
use crate::{error, PayloadContent, PayloadLocal, Uploader};

impl Uploader {
    #[instrument(skip_all, fields(
        content_type = field.content_type(),
        qiniu.bucket = local.bucket(),
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

pub struct ByteUploader<L>
where
    L: PayloadLocal,
{
    field: Bytes,
    content_type: Option<SmallString<[u8; 64]>>,
    local: L,
}

impl<L> ByteUploader<L>
where
    L: PayloadLocal,
{
    pub async fn from_field(
        field: Field<'_>, local: L,
    ) -> Result<Self, MultipartError> {
        let content_type = field.content_type().map(SmallString::from_str);
        let payload = field.bytes().await?;
        Ok(Self {
            field: payload,
            content_type,
            local,
        })
    }
}

impl<L> PayloadContent for ByteUploader<L>
where
    L: PayloadLocal,
{
    type Payload = Cursor<Bytes>;

    fn content_type(&self) -> mime::Mime {
        self.content_type
            .as_deref()
            .and_then(|v| v.parse().ok())
            .unwrap_or(APPLICATION_OCTET_STREAM)
    }

    fn payload(self) -> Result<Self::Payload, crate::error::Error> {
        Ok(Cursor::new(self.field))
    }
}

impl<L> PayloadLocal for ByteUploader<L>
where
    L: PayloadLocal,
{
    fn bucket(&self) -> &str { self.local.bucket() }

    fn obj_name(&self) -> &str { self.local.obj_name() }

    fn file_name(&self) -> &str { self.local.obj_name() }
}
