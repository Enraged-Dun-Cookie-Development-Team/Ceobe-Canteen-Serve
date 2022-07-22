use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;

pub mod checkers;
pub mod models;
pub mod operate;

#[derive(Debug, Error)]
pub enum UserError {
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

impl status_err::StatusErr for UserError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            UserError::Db(db) => db.prefix(),
            UserError::UsernameLength(rang) => rang.prefix(),
            UserError::UserNotExist
            | UserError::ConflictUsername { username: _ }
            | UserError::PasswordNoChange
            | UserError::PasswordWrong => ErrPrefix::UNAUTHORIZED,
        }
    }

    fn code(&self) -> u16 {
        match self {
            UserError::Db(db) => db.code(),
            UserError::UsernameLength(l) => l.code(),
            UserError::PasswordWrong => 0x0004,
            UserError::UserNotExist => 0x00_07,
            UserError::ConflictUsername { username: _ } => 0x0008,
            UserError::PasswordNoChange => 0x0009,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            UserError::Db(db) => db.http_code(),
            UserError::UsernameLength(l) => l.http_code(),
            UserError::PasswordWrong => HttpCode::UNAUTHORIZED,
            UserError::UserNotExist => HttpCode::NOT_FOUND,
            UserError::ConflictUsername { username: _ }
            | UserError::PasswordNoChange => HttpCode::BAD_REQUEST,
        }
    }
}
