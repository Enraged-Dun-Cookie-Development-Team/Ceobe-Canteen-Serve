use status_err::StatusErr;
use thiserror::Error;

pub mod username;

#[derive(Debug, Error, PartialEq, Eq, StatusErr)]
pub enum CheckError {
    #[error("用户名长度范围不达标: {0}")]
    UsernameLength(#[from] range_limit::Error),
}
