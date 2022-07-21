use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;

pub mod checkers;
pub mod operate;
pub mod sql_models;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("SQL数据库异常")]
    Db(#[from] sea_orm::DbErr),
    #[error("用户名长度范围不达标: {0}")]
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
            CommonError::UserNotExist
            | CommonError::ConflictUsername { username: _ }
            | CommonError::PasswordNoChange
            | CommonError::PasswordWrong => ErrPrefix::UNAUTHORIZED,
        }
    }

    fn code(&self) -> u16 {
        match self {
            CommonError::Db(db) => db.code(),
            CommonError::UsernameLength(l) => l.code(),
            CommonError::PasswordWrong => 0x0004,
            CommonError::UserNotExist => 0x00_07,
            CommonError::ConflictUsername { username: _ } => 0x0008,
            CommonError::PasswordNoChange => 0x0009,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            CommonError::Db(db) => db.http_code(),
            CommonError::UsernameLength(l) => l.http_code(),
            CommonError::PasswordWrong => HttpCode::UNAUTHORIZED,
            CommonError::UserNotExist => HttpCode::NOT_FOUND,
            CommonError::ConflictUsername { username: _ }
            | CommonError::PasswordNoChange => HttpCode::BAD_REQUEST,
        }
    }
}
