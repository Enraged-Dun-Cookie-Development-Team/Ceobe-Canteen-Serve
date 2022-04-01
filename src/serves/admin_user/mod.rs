use resp_result::RespResult;

pub mod controllers;
mod error;

type AdminUserRResult<T> = RespResult<T, error::AdminUserError>;