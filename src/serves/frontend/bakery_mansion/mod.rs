use axum_prehandle::PreRespHandling;
use resp_result::FlagRespResult;

pub mod controllers;
pub(crate) mod error;
mod models;
mod view;

type FLagMansionRResult<T> = FlagRespResult<Option<T>, error::MansionError>;

use self::models::MidCheckerPretreat;

pub type MidCheckerPretreatment = PreRespHandling<MidCheckerPretreat>;
