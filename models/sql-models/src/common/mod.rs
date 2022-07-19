use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;

pub mod checkers;
pub mod operate;
pub mod sql_models;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("SQL数据库异常")]
    Db(#[from] sea_orm::DbErr),
    #[error("用户名长度范围不达标")]
    UsernameLength(#[from] range_limit::Error),
    #[error("指定用户不存在")]
    UserNotExist,
    #[error("用户名冲突，[{username:?}]已经被使用")]
    ConflictUsername { username: String },
    #[error("密码未更改")]
    PasswordNoChange,
    #[error("密码校验错误")]
    PasswordWrong,
}

impl status_err::StatusErr for CommonError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            CommonError::Db(db) => db.prefix(),
            CommonError::UsernameLength(rang) => rang.prefix(),
            CommonError::UserNotExist => ErrPrefix::UNAUTHORIZED,
            CommonError::ConflictUsername { username: _ } => {
                ErrPrefix::UNAUTHORIZED
            }
            CommonError::PasswordNoChange => ErrPrefix::UNAUTHORIZED,
            CommonError::PasswordWrong => ErrPrefix::UNAUTHORIZED,
        }
    }

    fn code(&self) -> u16 {
        match self {
            CommonError::Db(db) => db.code(),
            CommonError::UsernameLength(l) => l.code(),
            CommonError::UserNotExist => 0x0007,
            CommonError::ConflictUsername { username: _ } => 0x0008,
            CommonError::PasswordNoChange => 0x0009,
            CommonError::PasswordWrong => 0x0004,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            CommonError::Db(db) => db.http_code(),
            CommonError::UsernameLength(l) => l.http_code(),
            CommonError::UserNotExist => HttpCode::NOT_FOUND,
            CommonError::ConflictUsername { username: _ } => {
                HttpCode::BAD_REQUEST
            }
            CommonError::PasswordNoChange => HttpCode::BAD_REQUEST,
            CommonError::PasswordWrong => HttpCode::UNAUTHORIZED,
        }
    }
}
