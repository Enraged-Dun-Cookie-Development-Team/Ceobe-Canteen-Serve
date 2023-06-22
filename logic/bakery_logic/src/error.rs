use std::convert::Infallible;

use bakery::mansion::OperateError as MansionOperateError;
use status_err::{StatusErr};
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error(transparent)]
    #[status_err(err = "transparent")]
    TempListOperateError(#[from] MansionOperateError),
}

impl From<Infallible> for LogicError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}

pub(crate) type LogicResult<T> = Result<T, LogicError>;
