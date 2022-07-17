use chrono::NaiveDate;

use super::{each_info::{EachInfoUncheck, InfoChecker}};
use crate::{
    serves::backend::bakery_mansion::error::MansionError,
    utils::{
        data_checker::{
            collect_checkers::iter_checkers::IntoIterChecker, DataChecker,
        },
        data_struct::MaxLimitString,
    }, models::mansion::check::{Daily, Info},
};

crate::check_obj! {
    #[derive(serde::Deserialize,Debug)]
    pub struct DailyUncheck = DailyChecker > Daily{
        #[serde(rename="datetime")]
        pub date_time:DateFormatChecker,
        pub content: MaxLimitString<2048>,
        pub info:IntoIterChecker<Vec<EachInfoUncheck>,InfoChecker,Vec<Info>>
    }
    err:MansionError
}
pub struct DateFormatChecker;

impl DataChecker for DateFormatChecker {
    type Args = ();
    type Checked = NaiveDate;
    type Err = MansionError;
    type Fut = futures_util::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let date = NaiveDate::parse_from_str(&uncheck, "%Y-%m-%d")
            .map_err(MansionError::from);

        futures_util::future::ready(date)
    }
}
