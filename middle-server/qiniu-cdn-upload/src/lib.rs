use ceobe_qiniu_upload::{
    Error, PayloadContent, PayloadLocal, ResponsePayload, Uploader,
};
use mime::Mime;
use update_payload::UploadPayload;
use update_source::UploadSource;

/// 上传是数据的目标位置
pub mod update_payload;

/// 上传的数据来源相关trait
pub mod update_source;

pub async fn upload<Payload>(
    uploader: &Uploader,
    source: <Payload::Source as UploadSource>::Source<'_>, local: Payload,
) -> Result<ResponsePayload, Error>
where
    Payload: update_payload::UploadPayload,
    <Payload::Source as UploadSource>::Error: Into<Error>,
{
    let upload = UploadWrap::<_>::new(source, local)
        .await
        .map_err(Into::into)?;
    uploader.upload(upload).await
}

struct UploadWrap<L>
where
    L: UploadPayload,
    <L::Source as UploadSource>::Error: Into<Error>,
{
    content_type: Mime,
    payload: <L::Source as update_source::UploadSource>::Read,
    full_name: String,
}

impl<L> UploadWrap<L>
where
    L: UploadPayload,
    <L::Source as UploadSource>::Error: Into<Error>,
{
    async fn new(
        payload: <L::Source as UploadSource>::Source<'_>, local: L,
    ) -> Result<Self, <L::Source as UploadSource>::Error> {
        Ok(Self {
            content_type: <L::Source as UploadSource>::content_type(&payload),
            payload: <L::Source as UploadSource>::read_data(payload).await?,
            full_name: local.full_name(),
        })
    }
}

impl<L> PayloadContent for UploadWrap<L>
where
    L: UploadPayload,
    <L::Source as UploadSource>::Error: Into<Error>,
{
    type Payload = <L::Source as UploadSource>::Read;

    fn content_type(&self) -> Mime { self.content_type.clone() }

    fn payload(self) -> Result<Self::Payload, Error> { Ok(self.payload) }
}

impl<L> PayloadLocal for UploadWrap<L>
where
    L: UploadPayload,
    <L::Source as UploadSource>::Error: Into<Error>,
{
    fn obj_name(&self) -> &str { self.full_name.as_str() }
}
