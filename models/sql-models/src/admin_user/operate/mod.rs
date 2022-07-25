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

#[derive(Debug, Error)]
pub enum OperateError {
    #[error("SQL数据库异常")]
    Db(#[from] sea_orm::DbErr),
    #[error("指定用户不存在")]
    UserNotExist,
    #[error("用户名冲突，[{username:?}]已经被使用")]
    ConflictUsername { username: String },
    #[error("密码未更改")]
    PasswordNoChange,
    #[error("密码校验错误")]
    PasswordWrong,
}

impl status_err::StatusErr for OperateError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            Db(db) => db.prefix(),
            UserNotExist
            | ConflictUsername { username: _ }
            | PasswordNoChange
            | PasswordWrong => ErrPrefix::UNAUTHORIZED,
        }
    }

    fn code(&self) -> u16 {
        match self {
            Db(db) => db.code(),
            PasswordWrong => 0x0004,
            UserNotExist => 0x00_07,
            ConflictUsername { username: _ } => 0x0008,
            PasswordNoChange => 0x0009,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            Db(db) => db.http_code(),
            PasswordWrong => HttpCode::UNAUTHORIZED,
            UserNotExist => HttpCode::NOT_FOUND,
            ConflictUsername { username: _ }
            | PasswordNoChange => HttpCode::BAD_REQUEST,
        }
    }
}
