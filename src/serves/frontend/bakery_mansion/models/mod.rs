use checker::QueryCheckExtract;
use mongo_migration::mongo_models::bakery::mansion::preludes::id_checker::MidChecker;

use super::error::MansionError;

pub type MidCheckerPretreatment = QueryCheckExtract<MidChecker, MansionError>;
