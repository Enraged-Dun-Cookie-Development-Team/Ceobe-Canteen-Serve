use resp_result::RespResult;

pub mod controllers;
mod error;
mod model;

type CeobeRResult<T> = RespResult<T, error::CeobeError>;
