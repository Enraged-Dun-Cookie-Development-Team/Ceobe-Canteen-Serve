use futures::{AsyncRead, Future};
use mime::Mime;
use std::{error::Error as StdError, fmt::Debug};

/// 上传七牛云的数据的数据源
pub trait UploadSource {
    /// 待上传的原始数据
    /// 可以包含除了原始数据的其他信息
    type Source<'r>: 'r;

    /// 用于上传时使用的 Read
    type Read: AsyncRead + Send + Sync + 'static + Debug;
    /// 提取上传数据时的异常
    type Error: StdError;

    type ReadFuture<'f>: Future<Output = Result<Self::Read, Self::Error>> + 'f + Send;

    fn read_data(payload: Self::Source<'_>) -> Self::ReadFuture<'_>;

    /// 上传数据的content type
    fn content_type(payload: &Self::Source<'_>) -> Mime;
}
