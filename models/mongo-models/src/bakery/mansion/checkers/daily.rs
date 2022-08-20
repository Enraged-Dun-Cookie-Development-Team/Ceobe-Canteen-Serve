use checker::{
    check_gen,
    prefabs::{
        collect_checkers::iter_checkers::IntoIterChecker,
        date_time_format::DateFormatChecker,
    },
};

use super::{
    each_info::{EachInfoUncheck, InfoChecker},
    CheckError, MaxLimitString,
};
use crate::bakery::mansion::preludes::{Daily, Info};

#[check_gen(uncheck = DailyUncheck, checked = Daily, error = CheckError)]
#[derive(serde::Deserialize, Debug)]
pub struct DailyChecker {
    #[serde(rename = "datetime")]
    pub date_time: DateFormatChecker,
    pub content: MaxLimitString<2048>,
    pub info: IntoIterChecker<Vec<EachInfoUncheck>, InfoChecker, Vec<Info>>,
}
