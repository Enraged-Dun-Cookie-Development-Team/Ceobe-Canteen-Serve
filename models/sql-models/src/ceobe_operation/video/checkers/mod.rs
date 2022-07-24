pub mod bv;
pub mod video_data;
use std::convert::Infallible;

use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;
pub use CeobeOperationVideoCheckError::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CeobeOperationVideoCheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Bv号错误: {0}")]
    WrongBv(String),

    #[error("不可能失败")]
    Infallible(#[from] Infallible),
}

impl StatusErr for CeobeOperationVideoCheckError {
    fn prefix(&self) -> status_err::ErrPrefix {
        match self {
            LengthExceed(inner) => inner.prefix(),
            WrongBv(_) => ErrPrefix::CHECKER,
            Infallible(_) => unreachable!(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            LengthExceed(inner) => inner.code(),
            WrongBv(_) => 0x00_09,
            Infallible(_) => unreachable!(),
        }
    }
}
