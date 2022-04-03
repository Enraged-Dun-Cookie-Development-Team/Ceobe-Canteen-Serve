use resp_result::RespResult;

mod controllers;
mod error;
mod model;

type CeobeRResult<T> = RespResult<T, error::CeobeError>;

pub use controllers::CeobeController;
pub use model::CeobePushModel;