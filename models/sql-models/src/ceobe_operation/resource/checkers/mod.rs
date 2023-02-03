pub mod countdown;
pub mod resource_all_available;
pub mod resource_data;

use status_err::StatusErr;
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("字符串长度未达标 {0}")]
    StrLengthExceed(#[from] range_limit::Error),

    #[error("日期格式不正确 {0}")]
    DateFormat(#[from] chrono::ParseError),
}
