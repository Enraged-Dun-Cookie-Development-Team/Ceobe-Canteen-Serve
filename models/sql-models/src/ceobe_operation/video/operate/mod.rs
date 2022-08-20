mod retrieve;
mod update;
use thiserror::Error;

pub struct CeobeOperationVideoSqlOperate;

pub use OperateError::*;

#[derive(Debug, Error, status_err::StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;
