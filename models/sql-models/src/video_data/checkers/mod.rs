use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VideoDataCheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Bv号错误: {0}")]
    WrongBv(String),
}

impl StatusErr for VideoDataCheckError {
    fn prefix(&self) -> status_err::ErrPrefix {
        match self {
            VideoDataCheckError::LengthExceed(inner) => inner.prefix(),
            VideoDataCheckError::WrongBv(_) => ErrPrefix::CHECKER,
        }
    }

    fn code(&self) -> u16 {
        match self {
            VideoDataCheckError::LengthExceed(inner) => inner.code(),
            VideoDataCheckError::WrongBv(_) => 0x00_09,
        }
    }
}
