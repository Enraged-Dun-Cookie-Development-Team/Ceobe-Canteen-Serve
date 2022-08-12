pub mod create;
pub mod retrieve;
pub mod verify;

use sea_orm::FromQueryResult;
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;

pub struct CeobeOperationAppVersionSqlOperate;

pub use OperateError::*;

#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("App指定版本:[{0:?}]信息已经存在")]
    #[status_err(err(
        err_code = 0x000B,
        prefix = "ErrPrefix::CHECKER",
        http_code = "HttpCode::CONFLICT"
    ))]
    AppVersionIdExist(String),
    #[error("App指定版本:[{0:?}]信息不存在")]
    #[status_err(err(err_code = 0x0004, prefix = "ErrPrefix::NOT_FOUND",))]
    AppVersionIdNoExist(String),
    #[error("还没有App版本信息")]
    #[status_err(err(err_code = 0x0005, prefix = "ErrPrefix::NOT_FOUND",))]
    NotAppVersion,
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

#[derive(FromQueryResult)]
struct AppVerionCounts {
    pub(crate) count: i64,
}
