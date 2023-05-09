use status_err::{ErrPrefix, StatusErr};
#[derive(Debug, thiserror::Error, StatusErr)]
#[status_err(resp_err)]
pub enum Error {
    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),
    #[error(transparent)]
    Status(#[from] tonic::Status),
    #[error("Push log to qq channel failure")]
    #[status_err(err(
        err_code = 0x0003,
        prefix = "ErrPrefix::LOGGER_REPORT",
        resp_msg = "发送日志时收到失败响应"
    ))]
    PushLogFailure,
}
