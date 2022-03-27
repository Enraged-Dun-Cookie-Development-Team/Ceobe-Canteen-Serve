use crate::utils::mongodb_utils::db_selector::MongoDbSelector;
use crate::utils::req_pretreatment::prefabs::{MapErr, ToRResult};
use crate::utils::req_pretreatment::ReqPretreatment;

use super::modules::mansion::{MIdCheckerPretreat, MansionCheckerPretreat};
use super::{
    error::MansionError,
    modules::{loading_model, mansion::OptionMidCheckerPretreat, MansionDb},
};

pub mod mansion;

crate::generate_controller!(
    MansionController,
    "/mansion",
    mansion::save_mansion,
    mansion::get_mansion,
    mansion::get_all_id,
    mansion::remove_mansion
);

crate::extra_module!(
    MansionController=>crate::generate_collection_register!(
        MansionDb=>loading_model
    )
);

type OptionMidCheckerPretreatment =
    ReqPretreatment<ToRResult<MapErr<OptionMidCheckerPretreat, MansionError>>>;
type MidCheckerPretreatment = ReqPretreatment<ToRResult<MapErr<MIdCheckerPretreat, MansionError>>>;

type MansionBodyCheckerPretreatment =
    ReqPretreatment<ToRResult<MapErr<MansionCheckerPretreat, MansionError>>>;

type MansionMongoDbPretreatment =
    ReqPretreatment<ToRResult<MapErr<MongoDbSelector<MansionDb>, MansionError>>>;
