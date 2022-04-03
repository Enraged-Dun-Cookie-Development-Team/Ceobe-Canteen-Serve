mod daily;
mod each_info;
mod id_checker;
mod mansion;

pub use daily::{Daily, DailyChecker, DailyUncheck};
pub use each_info::{EachInfoUncheck, Info, InfoChecker, Predict};
pub use id_checker::{
    MIdUncheck, MansionId, Mid, MidChecker, OpMIdUncheck, OpMidChecker,
    OptionMid,
};
pub use mansion::{Mansion, MansionChecker, MansionUncheck};
use request_pretreat::prefabs::DefaultValue;

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
