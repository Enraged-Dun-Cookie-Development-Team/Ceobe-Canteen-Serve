use resp_result::FlagRespResult;

pub mod controllers;
pub(crate) mod error;
mod models;
mod view;

type FlagMansionRResult<T> = FlagRespResult<Option<T>, error::MansionError>;
