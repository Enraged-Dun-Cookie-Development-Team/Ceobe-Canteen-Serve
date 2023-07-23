use status_err::StatusErr;
use thiserror::Error;
pub use CheckError::*;

pub mod global_config_data;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),
}
