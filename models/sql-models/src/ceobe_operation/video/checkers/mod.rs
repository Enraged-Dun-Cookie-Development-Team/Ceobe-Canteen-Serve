pub mod bv;
pub mod bv_arg_checker;
pub mod video_data;

use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;
pub use CheckError::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Bv号错误: {0:?}")]
    WrongBv(String),

    #[error("日期格式错误: {0}")]
    WrongDateTimeFormat(#[from] chrono::ParseError),
}

impl StatusErr for CheckError {
    fn prefix(&self) -> status_err::ErrPrefix {
        match self {
            LengthExceed(inner) => inner.prefix(),
            WrongBv(_) => ErrPrefix::CHECKER,
            WrongDateTimeFormat(inner) => inner.prefix(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            LengthExceed(inner) => inner.code(),
            WrongBv(_) => 0x00_09,
            WrongDateTimeFormat(inner) => inner.code(),
        }
    }
}