use super::{
    error::MansionError,
    models::checkers::{
        MIdCheckerPretreat, MansionCheckerPretreat, OptionMidCheckerPretreat,
    },
};
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

pub mod mansion;
pub mod mansion_front;

crate::generate_controller!(
    MansionController,
    "/mansion",
    mansion::save_mansion,
    mansion::get_mansion,
    mansion::get_recent_id,
    mansion::remove_mansion
);

crate::generate_controller!(
    MansionFrontController,
    "/bakery",
    mansion_front::get_mansion_with_time,
    mansion_front::get_all_id
);

crate::new_auth_level! {
    pub(super) MansionAuth=>[
        Chef
        Architect
    ]
}

type MansionAuthentication = AuthenticationLevel<MansionAuth, MansionError>;

type OptionMidCheckerPretreatment = ReqPretreatment<
    ToRResult<MapErr<OptionMidCheckerPretreat, MansionError>>,
>;
type MidCheckerPretreatment =
    ReqPretreatment<ToRResult<MapErr<MIdCheckerPretreat, MansionError>>>;

type MansionBodyCheckerPretreatment =
    ReqPretreatment<ToRResult<MapErr<MansionCheckerPretreat, MansionError>>>;

type MansionMongoDbPretreatment =
    ReqPretreatment<ToRResult<MapErr<MongoDbSelector, MansionError>>>;
