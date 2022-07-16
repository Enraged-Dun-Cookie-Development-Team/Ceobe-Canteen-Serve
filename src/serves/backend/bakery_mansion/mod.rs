use resp_result::RespResult;

pub mod controllers;
pub(crate) mod error;
mod models;
mod view;

type MansionRResult<T> = RespResult<T, error::MansionError>;

use crate::utils::{
    mongodb_utils::db_selector::MongoDbSelector,
    req_pretreatment::{
        prefabs::{MapErr, ToRResult},
        ReqPretreatment,
    },
    user_authorize::{
        auth_level::prefabs::{Architect, Chef},
        AuthenticationLevel,
    },
};

use self::{error::MansionError, models::checkers::{OptionMidCheckerPretreat, MIdCheckerPretreat, MansionCheckerPretreat}};

crate::new_auth_level! {
    pub MansionAuth=>[
        Chef
        Architect
    ]
}

pub type MansionAuthentication =
    AuthenticationLevel<MansionAuth, MansionError>;

pub type OptionMidCheckerPretreatment = ReqPretreatment<
    ToRResult<MapErr<OptionMidCheckerPretreat, MansionError>>,
>;
pub type MidCheckerPretreatment =
    ReqPretreatment<ToRResult<MapErr<MIdCheckerPretreat, MansionError>>>;

pub type MansionBodyCheckerPretreatment =
    ReqPretreatment<ToRResult<MapErr<MansionCheckerPretreat, MansionError>>>;

pub type MansionMongoDbPretreatment =
    ReqPretreatment<ToRResult<MapErr<MongoDbSelector, MansionError>>>;
