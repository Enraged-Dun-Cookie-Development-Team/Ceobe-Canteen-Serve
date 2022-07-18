mod daily;
mod each_info;
mod id_checker;
mod mansion;

use request_pretreat::prefabs::DefaultValue;

use self::id_checker::{MIdUncheck, MidChecker};
use crate::utils::{
    data_checker::{DataChecker, PretreatChecker},
    req_pretreatment::prefabs::Query,
};

pub type MIdCheckerPretreat = PretreatChecker<
    DefaultValue<<MidChecker as DataChecker>::Args>,
    Query<MIdUncheck>,
    MidChecker,
>;
