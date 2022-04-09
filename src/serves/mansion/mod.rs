use resp_result::RespResult;

mod controllers;
mod error;
mod models;
mod view;

type MansionRResult<T> = RespResult<T, error::MansionError>;


pub use controllers::MansionController;
pub use controllers::MansionFrontController;
