use std::convert::Infallible;

use persistence::bakery::mansion::OperateError as MansionOperateError;
use status_err::StatusErr;
use tencent_cloud_server::error::TcCloudError;
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error(transparent)]
    TempListOperateError(#[from] MansionOperateError),

    #[error(transparent)]
    TcCloudError(#[from] TcCloudError),

    #[error(transparent)]
    SerdeQs(#[from] serde_qs::Error),
}

impl From<Infallible> for LogicError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}

pub(crate) type LogicResult<T> = Result<T, LogicError>;
