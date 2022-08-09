pub mod resource_data;

use std::convert::Infallible;

use status_err::StatusErr;
use thiserror::Error;
pub use CheckError::*;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error(transparent)]
    StrLengthExceed(#[from] range_limit::Error),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!() }
}
