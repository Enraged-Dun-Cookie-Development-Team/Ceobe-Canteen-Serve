pub mod create;
pub mod retrieve;
pub mod update;

use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;

pub struct CeobeOperationResourceSqlOperate;

pub use OperateError::*;

#[derive(Debug, Error)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

impl StatusErr for OperateError {
    fn prefix(&self) -> ErrPrefix {
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