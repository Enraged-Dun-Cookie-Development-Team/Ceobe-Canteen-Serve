use chrono::NaiveDate;
use checker::{check_obj, Checker, prefabs::collect_checkers::iter_checkers::IntoIterChecker};
use range_limit::{limits::max_limit::MaxLimit, RangeBoundLimit};
use crate::{ mansion::{check::{Daily, Info}, MansionDataError},
};
use futures_util;

use super::each_info::{EachInfoUncheck, InfoChecker};

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;
check_obj! {
    #[derive(serde::Deserialize,Debug)]
    pub struct DailyUncheck = DailyChecker > Daily{
        #[serde(rename="datetime")]
        pub date_time:DateFormatChecker,
        pub content: MaxLimitString<2048>,
        pub info:IntoIterChecker<Vec<EachInfoUncheck>,InfoChecker,Vec<Info>>
    }
    err:MansionDataError
}
pub struct DateFormatChecker;

impl Checker for DateFormatChecker {
    type Args = ();
    type Checked = NaiveDate;
    type Err = MansionDataError;
    type Fut = futures_util::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let date = NaiveDate::parse_from_str(&uncheck, "%Y-%m-%d")
            .map_err(MansionDataError::from);

        futures_util::future::ready(date)
    }
}
