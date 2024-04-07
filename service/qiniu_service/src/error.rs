use ceobe_qiniu_upload::Error as QiniuError;
use mongodb::bson;
use redis::RedisError;
use status_err::StatusErr;
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum ServiceError {
    #[error(transparent)]
    #[status_err(err = "transparent")]
    Upload(#[from] QiniuError),

    #[error(transparent)]
    QqChannel(#[from] qq_channel_warning::Error),

    #[error("Redis异常: {0}")]
    Redis(#[from] RedisError),


}

pub(crate) type ServiceResult<T> = Result<T, ServiceError>;
