use checker::{JsonCheckExtract, QueryCheckExtract};
use persistence::bakery::models::mansion::preludes::id_checker::{MidChecker, OpMidChecker};
use persistence::bakery::models::mansion::preludes::mansion::MansionChecker;


use crate::serves::backend::bakery_mansion::error::MansionError;

pub type OptionMidCheckerPretreatment =
    QueryCheckExtract<OpMidChecker, MansionError>;
pub type MidCheckerPretreatment = QueryCheckExtract<MidChecker, MansionError>;

pub type MansionBodyCheckerPretreatment =
    JsonCheckExtract<MansionChecker, MansionError>;
