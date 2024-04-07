use ceobe_qiniu_upload::{
    Error, Manager, PayloadContent, PayloadLocal, ResponsePayload,
};
use mime::Mime;
use update_payload::UploadPayload;
use update_source::UploadSource;

/// 上传是数据的目标位置
pub mod update_payload;

/// 上传的数据来源相关trait
pub mod update_source;

#[deprecated]
/// 通用七牛云上传接口
///
/// - `uploader`: [`&Uploader`](Uploader) 七牛云上传器，可以通过
///   [`QiniuManager`](ceobe_qiniu_upload::QiniuManager)
/// 解引用获得
/// - `source`: 原始待上传信息，可以从中获得待上传的内容和待上传的
///   `Content-Type`
/// - `payload`: 指定荷载的信息，包括 上传七牛云的路径，如何从source
///   获取上传内容和`Content-Type`
///
/// # Errors
///
/// This function will return an error if
/// 1. 上传数据到七牛云时出现异常
/// 2. json 序列化/反序列化异常
/// 3. 读取 Multipart 时异常
pub async fn upload<Payload>(
    uploader: &Manager,
    source: <Payload::Source as UploadSource>::Source<'_>, payload: Payload,
) -> Result<ResponsePayload, Error>
where
    Payload: update_payload::UploadPayload,
    <Payload::Source as UploadSource>::Error: Into<Error>,
{
    let upload = UploadWrap::<_>::new(source, payload)
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
