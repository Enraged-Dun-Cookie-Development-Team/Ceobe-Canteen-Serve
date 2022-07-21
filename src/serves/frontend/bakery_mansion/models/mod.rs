use mongo_migration::mongo_models::mansion::checkers::id_checker::{
    MidChecker, MidUncheck,
};
use request_pretreat::prefabs::QueryArgs;

use crate::{
    serves::frontend::bakery_mansion::error::MansionError,
    utils::data_checker::PreLiteChecker,
};

pub type MidCheckerPretreat =
    PreLiteChecker<QueryArgs<MidUncheck>, MidChecker, MansionError>;
