
use sql_models::fetcher::global_config::operate::OperateError;
use status_err::StatusErr;
use thiserror::Error;


#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error("数据库操作异常: {0}")]
    DbOperateError(#[from] OperateError),
}

#[allow(dead_code)]
pub(crate) type LogicResult<T> = Result<T, LogicError>;