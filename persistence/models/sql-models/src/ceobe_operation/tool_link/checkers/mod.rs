pub mod tags;
use std::convert::Infallible;

use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;
pub use CheckError::*;

pub mod tool_link_data;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),

    #[error("Datasource Unique key[{0:?}] 未找到")]
    #[status_err(err(prefix = "ErrPrefix::CHECKER", err_code = 0x0017u16))]
    UniqueKeyInvalid(String),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    Json(#[from] serde_json::Error),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}
