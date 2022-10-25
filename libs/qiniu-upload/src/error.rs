use smallstr::SmallString;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    QiniuHttp(#[from] qiniu_http_client::ResponseError),
    #[error("Bucket[{0:?}] not managed")]
    BucketNotInManage(SmallString<[u8; 64]>),
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("Custom Error: {0:?}")]
    Custom(String),
}

#[derive(Debug, thiserror::Error)]
#[error("Qiniu Uploader not found, did you add it to extension?")]
pub struct UploaderNotFound;
