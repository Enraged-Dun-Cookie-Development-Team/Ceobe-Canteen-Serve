use axum::extract::multipart::MultipartError;
use qiniu_upload_manager::apis::http_client::ResponseError;
use status_err::ErrPrefix;

#[derive(Debug, thiserror::Error, status_err::StatusErr)]
pub enum Error {
    #[error(transparent)]
    #[status_err(err(
        prefix = "ErrPrefix::QI_NIU",
        err_code = 0x0001,
        resp_msg = "上传七牛云时出现异常"
    ))]
    QiniuHttp(#[from] ResponseError),
    #[error(transparent)]
    #[status_err(err = "transparent")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    Multipart(#[from] MultipartError),
}

#[derive(Debug, thiserror::Error)]
#[error("Qiniu Uploader not found, did you add it to extension?")]
pub struct UploaderNotFound;
