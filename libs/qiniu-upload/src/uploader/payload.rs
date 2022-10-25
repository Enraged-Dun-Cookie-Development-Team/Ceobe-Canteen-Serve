use crate::error;
use futures::AsyncRead;
use mime::Mime;
use mime_guess::{mime::APPLICATION_OCTET_STREAM, MimeGuess};
use std::{fmt::Debug, path::Path};

pub trait PayloadLocal {
    fn bucket(&self) -> &str;

    fn obj_name(&self) -> &str;

    fn file_name(&self) -> &str {
        self.obj_name()
    }
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
