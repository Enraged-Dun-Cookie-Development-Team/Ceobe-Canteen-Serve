pub mod app_version_data;
pub mod app_version_checker;

use std::convert::Infallible;

use thiserror::Error;

use status_err::{ErrPrefix, StatusErr};
pub use CheckError::*;

#[derive(Debug, Error)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("版本号错误: {0:?}")]
    VersionFormat(String),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}

impl StatusErr for CheckError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            LengthExceed(inner) => inner.prefix(),
            VersionFormat(_) => ErrPrefix::CHECKER,
        }
    }

    fn code(&self) -> u16 {
        match self {
            LengthExceed(inner) => inner.code(),
            VersionFormat(_) => 0x000A,
        }
    }
}
