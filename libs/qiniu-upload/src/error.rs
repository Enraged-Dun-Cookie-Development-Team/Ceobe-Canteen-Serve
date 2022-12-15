use axum::extract::multipart::MultipartError;
use qiniu_upload_manager::apis::http_client::ResponseError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    QiniuHttp(#[from] ResponseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Multipart(#[from] MultipartError),

    #[error("Custom Error: {0:?}")]
    Custom(String),
}

#[derive(Debug, thiserror::Error)]
#[error("Qiniu Uploader not found, did you add it to extension?")]
pub struct UploaderNotFound;
