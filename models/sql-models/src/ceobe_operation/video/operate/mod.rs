use thiserror::Error;

pub struct CeoboOperationVideoSqlOperate;

pub use CeoboOperationVideoOperateError::*;

#[derive(Debug, Error)]
pub enum CeoboOperationVideoOperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, CeoboOperationVideoOperateError>;

impl status_err::StatusErr for CeoboOperationVideoOperateError {
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
