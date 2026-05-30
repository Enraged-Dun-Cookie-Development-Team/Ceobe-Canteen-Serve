use axum::extract::multipart::MultipartError;
use qiniu_upload_manager::apis::http_client::ResponseError;
use status_err::generated_error::qi_niu_kind::QiniuHttpError;

#[derive(Debug, thiserror::Error, status_err::StatusErr)]
// TODO: 后续修复 large_enum_variant，考虑 Box<ResponseError>
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[error(transparent)]
    #[status_err(err(bind = "QiniuHttpError"))]
    QiniuHttp(#[from] ResponseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Multipart(#[from] MultipartError),
}

#[derive(Debug, thiserror::Error)]
#[error("Qiniu Uploader not found, did you add it to extension?")]
pub struct UploaderNotFound;
