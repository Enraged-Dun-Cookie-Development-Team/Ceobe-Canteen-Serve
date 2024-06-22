use resp_result::RespResult;

pub mod controllers;
pub(crate) mod error;
mod models;

type MansionRResult<T> = RespResult<T, error::MansionError>;
