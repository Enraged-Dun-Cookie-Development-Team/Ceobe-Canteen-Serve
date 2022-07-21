use mongo_migration::mongo_models::mansion::checkers::{
    id_checker::{MidChecker, MidUncheck, OpMidChecker, OpMidUncheck},
    mansion::{MansionChecker, MansionUncheck},
};
use request_pretreat::prefabs::{JsonPayload, QueryArgs};

use crate::{
    serves::backend::bakery_mansion::error::MansionError,
    utils::data_checker::PreLiteChecker,
};

pub type MansionCheckerPretreat =
    PreLiteChecker<JsonPayload<MansionUncheck>, MansionChecker, MansionError>;

pub type MIdCheckerPretreat =
    PreLiteChecker<QueryArgs<MidUncheck>, MidChecker, MansionError>;
pub type OptionMidCheckerPretreat =
    PreLiteChecker<QueryArgs<OpMidUncheck>, OpMidChecker, MansionError>;
