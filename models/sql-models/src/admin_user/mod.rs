use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;

pub mod checkers;
pub mod models;
pub mod operate;

#[derive(Debug, Error)]
pub enum AdminUserError {
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

impl status_err::StatusErr for AdminUserError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            AdminUserError::Db(db) => db.prefix(),
            AdminUserError::UsernameLength(rang) => rang.prefix(),
            AdminUserError::UserNotExist
            | AdminUserError::ConflictUsername { username: _ }
            | AdminUserError::PasswordNoChange
            | AdminUserError::PasswordWrong => ErrPrefix::UNAUTHORIZED,
        }
    }

    fn code(&self) -> u16 {
        match self {
            AdminUserError::Db(db) => db.code(),
            AdminUserError::UsernameLength(l) => l.code(),
            AdminUserError::PasswordWrong => 0x0004,
            AdminUserError::UserNotExist => 0x00_07,
            AdminUserError::ConflictUsername { username: _ } => 0x0008,
            AdminUserError::PasswordNoChange => 0x0009,
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            AdminUserError::Db(db) => db.http_code(),
            AdminUserError::UsernameLength(l) => l.http_code(),
            AdminUserError::PasswordWrong => HttpCode::UNAUTHORIZED,
            AdminUserError::UserNotExist => HttpCode::NOT_FOUND,
            AdminUserError::ConflictUsername { username: _ }
            | AdminUserError::PasswordNoChange => HttpCode::BAD_REQUEST,
        }
    }
}
