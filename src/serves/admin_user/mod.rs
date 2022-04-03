use resp_result::RespResult;

mod checker;
mod controllers;
mod error;
mod view;

type AdminUserRResult<T> = RespResult<T, error::AdminUserError>;

pub use controllers::AdminUserController;