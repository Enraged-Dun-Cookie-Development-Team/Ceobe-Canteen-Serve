mod update;
mod retrieve;
mod delete;

use status_err::HttpCode;
use thiserror::Error;

pub struct CeoboOperationAnnouncementSqlOperate;

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
            Db(db) => db.prefix(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            Db(db) => db.code(),
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            Db(db) => db.http_code(),
        }
    }
}