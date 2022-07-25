pub mod username;

use std::convert::Infallible;

use status_err::{ErrPrefix, StatusErr, HttpCode};
use thiserror::Error;
pub use CheckError::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CheckError {
    #[error("用户名长度范围不达标: {0}")]
    UsernameLength(#[from] range_limit::Error),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}

impl StatusErr for CheckError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            UsernameLength(inner) => inner.prefix(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            UsernameLength(inner) => inner.code(),
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            UsernameLength(l) => l.http_code(),
        }
    }
}
