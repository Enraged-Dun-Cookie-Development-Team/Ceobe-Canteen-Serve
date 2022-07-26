pub mod username;

use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;
pub use CheckError::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CheckError {
    #[error("用户名长度范围不达标: {0}")]
    UsernameLength(#[from] range_limit::Error),
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
