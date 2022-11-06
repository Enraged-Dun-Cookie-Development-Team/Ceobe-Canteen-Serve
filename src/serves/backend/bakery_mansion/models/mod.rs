use checker::{JsonCheckExtract, QueryCheckExtract};
use mongo_migration::mongo_models::bakery::mansion::checkers::{
    id_checker::{MidChecker, OpMidChecker},
    mansion::MansionChecker,
};

use crate::serves::backend::bakery_mansion::error::MansionError;

pub type OptionMidCheckerPretreatment =
    QueryCheckExtract<OpMidChecker, MansionError>;
pub type MidCheckerPretreatment = QueryCheckExtract<MidChecker, MansionError>;

pub type MansionBodyCheckerPretreatment =
    JsonCheckExtract<MansionChecker, MansionError>;
