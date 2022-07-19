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
