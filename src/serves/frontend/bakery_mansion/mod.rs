use axum_prehandle::PreRespHandling;
use resp_result::RespResult;

pub mod controllers;
pub(crate) mod error;
mod models;
mod view;

type MansionRResult<T> = RespResult<T, error::MansionError>;

use self::models::MidCheckerPretreat;

pub type MidCheckerPretreatment = PreRespHandling<MidCheckerPretreat>;
