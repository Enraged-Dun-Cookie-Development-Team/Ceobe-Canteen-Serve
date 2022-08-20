use axum_prehandle::PreRespHandling;
use resp_result::RespResult;

pub mod controllers;
pub(crate) mod error;
mod models;
mod view;

type MansionRResult<T> = RespResult<T, error::MansionError>;

use self::models::{
    MIdCheckerPretreat, MansionCheckerPretreat, OptionMidCheckerPretreat,
};

pub type OptionMidCheckerPretreatment =
    PreRespHandling<OptionMidCheckerPretreat>;
pub type MidCheckerPretreatment = PreRespHandling<MIdCheckerPretreat>;

pub type MansionBodyCheckerPretreatment =
    PreRespHandling<MansionCheckerPretreat>;
