use resp_result::RespResult;

pub mod controllers;
mod db_ops;
mod error;
mod modules;
mod pretreatments;

type MansionRResult<T> = RespResult<T, error::MansionError>;
