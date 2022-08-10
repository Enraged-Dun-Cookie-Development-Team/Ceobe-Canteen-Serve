pub mod create;
pub mod retrieve;
pub mod verify;

use sea_orm::FromQueryResult;
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;

pub struct CeobeOperationAppVersionSqlOperate;

pub use OperateError::*;

#[derive(Debug, Error)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("App指定版本:[{0:?}]信息已经存在")]
    AppVersionIdExist(String),
    #[error("App指定版本:[{0:?}]信息不存在")]
    AppVersionIdNoExist(String),
    #[error("还没有App版本信息")]
    NotAppVersion,
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

impl StatusErr for OperateError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            Db(inner) => inner.prefix(),
            AppVersionIdExist(_) => ErrPrefix::CHECKER,
            AppVersionIdNoExist(_) => ErrPrefix::CHECKER,
            NotAppVersion=> ErrPrefix::CHECKER,
        }
    }

    fn code(&self) -> u16 {
        match self {
            Db(inner) => inner.code(),
            AppVersionIdExist(_) => 0x000B,
            AppVersionIdNoExist(_) => 0x000C,
            NotAppVersion=> 0x000D,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            AppVersionIdExist(_) => HttpCode::CONFLICT,
            _ => self.status().http_code(),
        }
    }
}

#[derive(FromQueryResult)]
struct AppVerionCounts {
    pub(crate) count: i64,
}
