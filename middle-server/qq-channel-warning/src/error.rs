use status_err::{
    StatusErr, generated_error::logger_report_kind::PushLogFailureError,
};

#[derive(Debug, thiserror::Error, StatusErr)]
#[status_err(resp_err)]
pub enum Error {
    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),
    #[error(transparent)]
    Status(#[from] tonic::Status),
    #[error("Push log to qq channel failure")]
    #[status_err(err(bind = "PushLogFailureError"))]
    PushLogFailure,
}
