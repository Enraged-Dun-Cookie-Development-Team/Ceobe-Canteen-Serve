mod daily;
mod each_info;
mod id_checker;
mod mansion;

use request_pretreat::prefabs::DefaultValue;

use self::{
    id_checker::{MIdUncheck, MidChecker, OpMIdUncheck, OpMidChecker},
    mansion::{MansionChecker, MansionUncheck},
};
use crate::utils::{
    data_checker::{DataChecker, PretreatChecker},
    req_pretreatment::prefabs::{Json, Query},
};

pub type MansionCheckerPretreat = PretreatChecker<
    DefaultValue<<MansionChecker as DataChecker>::Args>,
    Json<MansionUncheck>,
    MansionChecker,
>;

pub type MIdCheckerPretreat = PretreatChecker<
    DefaultValue<<MidChecker as DataChecker>::Args>,
    Query<MIdUncheck>,
    MidChecker,
>;
pub type OptionMidCheckerPretreat = PretreatChecker<
    DefaultValue<<OpMidChecker as DataChecker>::Args>,
    Query<OpMIdUncheck>,
    OpMidChecker,
>;

