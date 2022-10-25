use futures::io::Cursor;
use mime::Mime;
use serde::Serialize;

use crate::{error, Uploader};

use super::{
    payload::{PayloadContent, PayloadLocal},
    ResponsePayload,
};

impl Uploader {
    pub async fn upload_json(
        &self,
        payload: impl JsonPayload + PayloadLocal,
    ) -> Result<ResponsePayload, error::Error> {
        let payload = JsonUpload::new(payload);

        self.upload(payload).await
    }
}

pub trait JsonPayload {
    type Payload: Serialize;

    fn payload(self) -> Self::Payload;
}

struct JsonUpload<P> {
    payload: P,
}

impl<P: JsonPayload> PayloadContent for JsonUpload<P> {
    fn content_type(&self) -> Mime {
        "application/json; charset=utf-8".parse().unwrap()
    }

    type Payload = Cursor<Vec<u8>>;

    fn payload(self) -> Result<Self::Payload, crate::error::Error> {
        let payload = self.payload.payload();
        let slice = serde_json::to_vec(&payload)?;
        Ok(Cursor::new(slice))
    }
}

impl<P> JsonUpload<P> {
    fn new(payload: P) -> Self {
        Self { payload }
    }
}

impl<P: PayloadLocal> PayloadLocal for JsonUpload<P> {
    fn bucket(&self) -> &str {
        self.payload.bucket()
    }

    fn obj_name(&self) -> &str {
        self.payload.obj_name()
    }

    fn file_name(&self) -> &str {
        self.payload.file_name()
    }
}
