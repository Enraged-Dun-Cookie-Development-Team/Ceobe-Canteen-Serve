pub mod create;
pub mod retrieve;
pub mod update;

use status_err::StatusErr;
use thiserror::Error;

pub struct CeobeOperationResourceSqlOperate;
use status_err::{ErrPrefix, HttpCode};
pub use OperateError::*;
#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("存在多个可用的资源全可用的记录")]
    #[status_err(err(
        prefix = r#"ErrPrefix::CHECKER"#,
        err_code = 0x00_0D,
        http_code = "HttpCode::INTERNAL_SERVER_ERROR"
    ))]
    MultiAllAvailable,
    #[error("没有可用的资源全可用的记录")]
    #[status_err(err(
        prefix = r#"ErrPrefix::CHECKER"#,
        err_code = 0x00_0E,
        http_code = "HttpCode::INTERNAL_SERVER_ERROR"
    ))]
    NoneAllAvailable,
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;
