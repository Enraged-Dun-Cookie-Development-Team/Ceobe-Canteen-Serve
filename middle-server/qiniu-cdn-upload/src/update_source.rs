use std::{error::Error as StdError, fmt::Debug, pin::Pin};

use axum::{
    body::Bytes,
    extract::multipart::{Field, MultipartError},
};
use futures::{io::Cursor, AsyncRead, Future};
use mime::{Mime, APPLICATION_OCTET_STREAM};

/// 上传七牛云的数据的数据源
pub trait UploadSource {
    /// 待上传的原始数据
    /// 可以包含除了原始数据的其他信息
    type Source<'r>: 'r;

    /// 用于上传时使用的 Read
    type Read: AsyncRead + Send + Sync + 'static + Debug;
    /// 提取上传数据时的异常
    type Error: StdError;

    type ReadFuture<'f>: Future<Output = Result<Self::Read, Self::Error>>
        + 'f
        + Send;

    /// 从 [Self::Source] 中获取payload, 可能为异步
    fn read_data(payload: Self::Source<'_>) -> Self::ReadFuture<'_>;

    /// 上传数据的content type
    fn content_type(payload: &Self::Source<'_>) -> Mime;
}

/// 来自 `Multipart` 的待上传数据源
pub struct FieldSource;

impl UploadSource for FieldSource {
    type Error = MultipartError;
    type Read = Cursor<Bytes>;
    type ReadFuture<'f> = Pin<
        Box<dyn Future<Output = Result<Self::Read, Self::Error>> + Send + 'f>,
    >;
    type Source<'r> = Field<'r>;

    fn read_data(payload: Self::Source<'_>) -> Self::ReadFuture<'_> {
        Box::pin(async move {
            let body = payload.bytes().await?;
            Ok(Cursor::new(body))
        })
    }

    fn content_type(payload: &Self::Source<'_>) -> Mime {
        payload
            .content_type()
            .and_then(|v| v.parse().ok())
            .unwrap_or(APPLICATION_OCTET_STREAM)
    }
}
