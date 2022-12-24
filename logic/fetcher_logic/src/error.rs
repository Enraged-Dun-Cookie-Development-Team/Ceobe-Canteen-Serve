
use sql_models::fetcher::global_config::{operate::OperateError, checkers::CheckError};
use status_err::StatusErr;
use thiserror::Error;


#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error(transparent)]
    #[status_err(err = "transparent")]
    OperateError(#[from] OperateError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    CheckErr(#[from] CheckError)
}

#[allow(dead_code)]
pub(crate) type LogicResult<T> = Result<T, LogicError>;