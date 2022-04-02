use resp_result::RespResult;

pub mod controllers;
mod error;
mod view;
mod checker;

type AdminUserRResult<T> = RespResult<T, error::AdminUserError>;