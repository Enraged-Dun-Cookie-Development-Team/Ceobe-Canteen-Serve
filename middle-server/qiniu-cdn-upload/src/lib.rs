use ceobe_qiniu_upload::{
    Error, PayloadContent, PayloadLocal, ResponsePayload, Uploader,
};
use mime::Mime;
use update_payload::UploadPayload;
use update_source::UploadSource;
use upload_bucket::UploadBucket;

/// 上传是数据的目标位置
pub mod update_payload;
pub mod upload_bucket;

/// 上传的数据来源相关trait
pub mod update_source;

pub async fn upload<Payload>(
    uploader: &Uploader,
    payload: <Payload::Source as UploadSource>::Payload<'_>, local: Payload,
) -> Result<ResponsePayload, Error>
where
    Payload: update_payload::UploadPayload,
    <Payload::Source as UploadSource>::Error: Into<Error>,
{
    let upload = UploadWrap::<_>::new(payload, local);
    Ok(uploader.upload(upload).await?)
}

struct UploadWrap<'r, L>
where
    L: UploadPayload,
    <L::Source as UploadSource>::Error: Into<Error>,
{
    payload: <L::Source as update_source::UploadSource>::Payload<'r>,
    inner: L,
}

impl<'r, L> UploadWrap<'r, L>
where
    L: UploadPayload,
    <L::Source as UploadSource>::Error: Into<Error>,
{
    fn new(
        payload: <L::Source as UploadSource>::Payload<'r>, local: L,
    ) -> Self {
        Self {
            payload,
            inner: local,
        }
    }
}

impl<'r, L> PayloadContent for UploadWrap<'r, L>
where
    L: UploadPayload,
    <L::Source as UploadSource>::Error: Into<Error>,
{
    fn content_type(&self) -> Mime {
        <L::Source as UploadSource>::content_type(&self.payload)
    }

    type Payload = <L::Source as UploadSource>::Read;

    fn payload(mut self) -> Result<Self::Payload, Error> {
        <L::Source as UploadSource>::read_data(&mut self.payload)
            .map_err(Into::into)
    }
}

impl<'r, L> PayloadLocal for UploadWrap<'r, L>
where
    L: UploadPayload,
    <L::Source as UploadSource>::Error: Into<Error>,
{
    fn bucket(&self) -> &str {
        <L::Bucket as UploadBucket>::BUCKET_NAME
    }

    fn obj_name(&self) -> &str {
        self.inner.obj_name()
    }
}
