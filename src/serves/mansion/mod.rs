use resp_result::RespResult;

mod controllers;
mod error;
mod modules;
mod view;

type MansionRResult<T> = RespResult<T, error::MansionError>;


pub use controllers::MansionController;
pub use modules::MansionModel;
