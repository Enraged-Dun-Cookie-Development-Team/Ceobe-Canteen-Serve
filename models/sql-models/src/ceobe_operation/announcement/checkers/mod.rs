pub mod announcement_data;
use std::convert::Infallible;

use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;
pub use CheckError::*;

#[derive(Debug, Error)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("不可能失败")]
    Infallible(#[from] Infallible),
}

impl StatusErr for CheckError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            LengthExceed(inner) => inner.prefix(),
            Infallible(_) => unreachable!(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            LengthExceed(inner) => inner.code(),
            Infallible(_) => unreachable!(),
        }
    }
}