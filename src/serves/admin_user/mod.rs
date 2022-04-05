use resp_result::RespResult;

mod checker;
mod controllers;
mod error;
mod models;
mod view;

type AdminUserRResult<T> = RespResult<T, error::AdminUserError>;

pub use controllers::AdminUserController;
pub use models::AdminUserModel;

pub mod sql_entities {
    pub use super::models::sql_models::{auth, user};
}
