use std::{fmt::Debug, path::Path};

use axum::extract::multipart::{Field, MultipartError};
use bytes::Bytes;
use futures::{io::Cursor, AsyncRead};
use mime::Mime;
use mime_guess::{mime::APPLICATION_OCTET_STREAM, MimeGuess};
use smallstr::SmallString;

use crate::error;

pub trait PayloadLocal {
    fn obj_name(&self) -> &str;

    fn file_name(&self) -> &str { self.obj_name() }
}

pub trait PayloadContent {
    fn content_type(&self) -> Mime;

    type Payload: AsyncRead + Send + Sync + 'static + Debug;
    fn payload(self) -> Result<Self::Payload, error::Error>;
}

pub trait FilePayload {
    fn content_type(&self) -> Mime {
        let path = self.file_path().as_ref();
        let guess = MimeGuess::from_path(path);

        guess.first().unwrap_or(APPLICATION_OCTET_STREAM)
    }

    type Path: AsRef<Path> + Send + Sync + ?Sized;

    fn file_path(&self) -> &Self::Path;
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

    fn obj_name(&self) -> &str { self.local.obj_name() }

    fn file_name(&self) -> &str { self.local.obj_name() }
}
