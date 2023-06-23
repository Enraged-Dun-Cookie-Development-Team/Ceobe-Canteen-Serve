use resp_result::{FlagRespResult, RespResult};

pub mod controllers;
pub(crate) mod error;
mod models;
mod view;

type FlagMansionRResult<T> = FlagRespResult<Option<T>, error::MansionError>;
type MansionRResult<T> = RespResult<T, error::MansionError>;
