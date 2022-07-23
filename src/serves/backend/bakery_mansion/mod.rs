use axum_prehandle::PreRespHandling;
use resp_result::RespResult;

pub mod controllers;
pub(crate) mod error;
mod models;
mod view;

type MansionRResult<T> = RespResult<T, error::MansionError>;

use self::{
    error::MansionError,
    models::{
        MIdCheckerPretreat, MansionCheckerPretreat, OptionMidCheckerPretreat,
    },
};
use crate::utils::user_authorize::{
    auth_level::prefabs::{Architect, Chef},
    AuthenticationLevel,
};

crate::new_auth_level! {
    pub MansionAuth=>[
        Chef
        Architect
    ]
}

pub type MansionAuthentication =
    AuthenticationLevel<MansionAuth, MansionError>;

pub type OptionMidCheckerPretreatment =
    PreRespHandling<OptionMidCheckerPretreat>;
pub type MidCheckerPretreatment = PreRespHandling<MIdCheckerPretreat>;

pub type MansionBodyCheckerPretreatment =
    PreRespHandling<MansionCheckerPretreat>;
