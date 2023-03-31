use ceobe_qiniu_upload::Error as QiniuError;
use status_err::StatusErr;
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum ServiceError {
    #[error(transparent)]
    #[status_err(err = "transparent")]
    Upload(#[from] QiniuError),
}

pub(crate) type ServiceResult<T> = Result<T, ServiceError>;
