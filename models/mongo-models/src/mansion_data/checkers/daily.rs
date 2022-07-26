use checker::{
    check_obj,
    prefabs::{
        collect_checkers::iter_checkers::IntoIterChecker,
        date_time_format::DateFormatChecker,
    },
};

use super::{
    each_info::{EachInfoUncheck, InfoChecker},
    CheckError, MaxLimitString,
};
use crate::mansion_data::checked::{Daily, Info};

check_obj! {
    #[derive(serde::Deserialize,Debug)]
    pub struct DailyUncheck = DailyChecker > Daily{
        #[serde(rename="datetime")]
        pub date_time: DateFormatChecker,
        pub content: MaxLimitString<2048>,
        pub info: IntoIterChecker<Vec<EachInfoUncheck>,InfoChecker,Vec<Info>>
    }
    err:CheckError
}
