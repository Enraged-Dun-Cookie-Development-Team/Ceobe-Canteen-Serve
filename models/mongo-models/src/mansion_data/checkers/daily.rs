use checker::{
    check_obj, prefabs::collect_checkers::iter_checkers::IntoIterChecker,
    Checker,
};
use chrono::NaiveDate;
use futures_util::{self, future::ready};

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
pub struct DateFormatChecker;

impl Checker for DateFormatChecker {
    type Args = ();
    type Checked = NaiveDate;
    type Err = CheckError;
    type Fut = futures_util::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready(
            NaiveDate::parse_from_str(&uncheck, "%Y-%m-%d")
                .map_err(Into::into),
        )
    }
}
