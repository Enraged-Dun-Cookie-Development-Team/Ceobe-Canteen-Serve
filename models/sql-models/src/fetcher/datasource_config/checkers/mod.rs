pub mod datasource_config_data;

use std::convert::Infallible;

use thiserror::Error;

use status_err::{ErrPrefix, StatusErr, HttpCode};
pub use CheckError::*;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}
