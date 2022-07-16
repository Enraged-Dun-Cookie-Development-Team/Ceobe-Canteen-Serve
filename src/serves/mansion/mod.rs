use resp_result::RespResult;

pub mod controllers;
mod error;
mod models;
mod view;

type MansionRResult<T> = RespResult<T, error::MansionError>;
