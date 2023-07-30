use std::convert::Infallible;

use checker::prefabs::version_checker::VersionInvalidError;
use status_err::StatusErr;
use thiserror::Error;

pub mod app_version_data;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    VersionInvalidError(#[from] VersionInvalidError),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}
