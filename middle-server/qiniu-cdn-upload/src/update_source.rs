use futures::AsyncRead;
use mime::Mime;
use std::{error::Error as StdError, fmt::Debug};

/// 上传七牛云的数据的数据源
pub trait UploadSource {
    /// 待上传的原始数据
    /// 可以包含除了原始数据的其他信息
    type Payload<'r>: 'r;

    /// 用于上传时使用的 Read
    type Read: AsyncRead + Send + Sync + 'static + Debug;
    /// 提取上传数据时的异常
    type Error: StdError;

    fn read_data(
        payload: &mut Self::Payload<'_>,
    ) -> Result<Self::Read, Self::Error>;

    /// 上传数据的content type
    fn content_type(payload: &Self::Payload<'_>)->Mime;
}
