mod retrieve;
mod update;
use thiserror::Error;

pub struct CeoboOperationVideoSqlOperate;

pub use OperateError::*;

#[derive(Debug, Error)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

impl status_err::StatusErr for OperateError {
    fn prefix(&self) -> status_err::ErrPrefix {
        match self {
            Db(inner) => inner.prefix(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            Db(inner) => inner.code(),
        }
    }
}
