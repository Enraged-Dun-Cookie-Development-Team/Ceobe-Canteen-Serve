mod retrieve;
mod update;
mod verify;
use sea_orm::FromQueryResult;

mod create;
use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;
pub struct UserSqlOperate;

pub use OperateError::*;

#[derive(FromQueryResult)]
struct UserCounts {
    pub(crate) count: i64,
}

#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

#[derive(Debug, Error, status_err::StatusErr)]
pub enum OperateError {
    #[error("SQL数据库异常")]
    Db(#[from] sea_orm::DbErr),
    #[error("指定用户不存在")]
    #[status_err(err(
        err_code = 0x00_07,
        prefix = "ErrPrefix::UNAUTHORIZED",
        http_code = "HttpCode::NOT_FOUND"
    ))]
    UserNotExist,
    #[error("用户名冲突，[{username:?}]已经被使用")]
    #[status_err(err(
        err_code = 0x0008,
        prefix = "ErrPrefix::UNAUTHORIZED",
        http_code = "HttpCode::BAD_REQUEST"
    ))]
    ConflictUsername { username: String },
    #[error("密码未更改")]
    #[status_err(err(
        err_code = 0x0009,
        prefix = "ErrPrefix::UNAUTHORIZED",
        http_code = "HttpCode::BAD_REQUEST"
    ))]
    PasswordNoChange,
    #[error("密码校验错误")]
    #[status_err(err(
        err_code = 0x0004,
        prefix = "ErrPrefix::UNAUTHORIZED",
        http_code = "HttpCode::UNAUTHORIZED"
    ))]
    PasswordWrong,
}
