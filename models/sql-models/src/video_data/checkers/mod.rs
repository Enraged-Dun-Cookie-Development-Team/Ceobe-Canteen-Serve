pub mod bv;
pub mod video_data;
use std::convert::Infallible;

use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VideoDataCheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Bv号错误: {0}")]
    WrongBv(String),

    #[error("不可能失败")]
    Infallible(#[from] Infallible),
}

impl StatusErr for VideoDataCheckError {
    fn prefix(&self) -> status_err::ErrPrefix {
        match self {
            VideoDataCheckError::LengthExceed(inner) => inner.prefix(),
            VideoDataCheckError::WrongBv(_) => ErrPrefix::CHECKER,
            VideoDataCheckError::Infallible(_) => unreachable!(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            VideoDataCheckError::LengthExceed(inner) => inner.code(),
            VideoDataCheckError::WrongBv(_) => 0x00_09,
            VideoDataCheckError::Infallible(_) => unreachable!(),
        }
    }
}
