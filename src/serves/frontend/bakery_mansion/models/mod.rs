use axum_prehandle::prefabs::query::QueryParams;
use mongo_migration::mongo_models::bakery::mansion::checkers::id_checker::{
    MidChecker, MidUncheck,
};

use crate::{
    serves::frontend::bakery_mansion::error::MansionError,
    utils::data_checker::PreLiteChecker,
};

pub type MidCheckerPretreat =
    PreLiteChecker<QueryParams<MidUncheck>, MidChecker, MansionError>;
