use checker::QueryCheckExtract;
use persistence::bakery::models::mansion::preludes::id_checker::MidChecker;

use super::error::MansionError;

pub type MidCheckerPretreatment = QueryCheckExtract<MidChecker, MansionError>;
