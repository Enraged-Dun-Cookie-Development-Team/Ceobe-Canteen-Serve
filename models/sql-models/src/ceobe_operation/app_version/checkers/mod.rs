pub mod app_version_checker;
pub mod app_version_data;

use std::convert::Infallible;

use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("版本号错误: {0:?}")]
    #[status_err(err(err_code = 0x000A, prefix = "ErrPrefix::CHECKER"))]
    VersionFormat(String),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}
