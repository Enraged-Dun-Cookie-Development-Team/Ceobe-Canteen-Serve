pub mod bv;
pub mod bv_arg_checker;
pub mod video_data;

use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;
pub use CheckError::*;

#[derive(Debug, Error, PartialEq, Eq, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Bv号错误: {0:?}")]
    #[status_err(err(prefix = "ErrPrefix::CHECKER", err_code = 0x00_09))]
    WrongBv(String),

    #[error("日期格式错误: {0}")]
    WrongDateTimeFormat(#[from] chrono::ParseError),
}
