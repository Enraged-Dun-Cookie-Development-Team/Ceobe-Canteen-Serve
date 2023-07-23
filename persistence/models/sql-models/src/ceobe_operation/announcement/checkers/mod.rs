use std::convert::Infallible;

use status_err::StatusErr;
use thiserror::Error;

pub mod announcement_data;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("日期格式错误: {0}")]
    DateTimeFormat(#[from] chrono::ParseError),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}
