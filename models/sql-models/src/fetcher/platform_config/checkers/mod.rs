pub mod platform_config_data;

use std::convert::Infallible;

use checker::prefabs::no_remainder_checker::HasRemError;
use status_err::StatusErr;
use thiserror::Error;
pub use CheckError::*;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error(transparent)]
    HasRem(#[from] HasRemError<1000>),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}
