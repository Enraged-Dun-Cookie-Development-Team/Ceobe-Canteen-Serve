use resp_result::RespResult;

mod checker;
pub mod controllers;
mod error;
mod view;

type AdminUserRResult<T> = RespResult<T, error::AdminUserError>;
