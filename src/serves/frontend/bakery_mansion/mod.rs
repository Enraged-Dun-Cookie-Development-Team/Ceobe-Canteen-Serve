use resp_result::RespResult;

pub mod controllers;
pub(crate) mod error;
mod models;
mod view;

type MansionRResult<T> = RespResult<T, error::MansionError>;

use self::models::MidCheckerPretreat;
use crate::utils::req_pretreatment::{prefabs::ToRResult, ReqPretreatment};

pub type MidCheckerPretreatment =
    ReqPretreatment<ToRResult<MidCheckerPretreat>>;
