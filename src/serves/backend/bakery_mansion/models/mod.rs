use axum_prehandle::prefabs::{json::JsonPayload, query::QueryParams};
use mongo_migration::mongo_models::mansion_data::checkers::{
    id_checker::{MidChecker, MidUncheck, OpMidChecker, OpMidUncheck},
    mansion::{MansionChecker, MansionUncheck},
};

use crate::{
    serves::backend::bakery_mansion::error::MansionError,
    utils::data_checker::PreLiteChecker,
};

pub type MansionCheckerPretreat =
    PreLiteChecker<JsonPayload<MansionUncheck>, MansionChecker, MansionError>;

pub type MIdCheckerPretreat =
    PreLiteChecker<QueryParams<MidUncheck>, MidChecker, MansionError>;
pub type OptionMidCheckerPretreat =
    PreLiteChecker<QueryParams<OpMidUncheck>, OpMidChecker, MansionError>;
