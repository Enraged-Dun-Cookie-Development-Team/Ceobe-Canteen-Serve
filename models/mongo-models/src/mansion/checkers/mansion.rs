use checker::{Checker, check_obj, prefabs::collect_checkers::iter_checkers::IntoIterChecker};
use futures::future::{err, ok, Ready};
use range_limit::{RangeBoundLimit, limits::max_limit::MaxLimit};
use serde::Deserialize;
use crate::mansion::MansionDataError;

use crate::mansion::check::{Mansion, Daily};

use super::{
    daily::{DailyChecker, DailyUncheck},
    id_checker::IdChecker,
};
type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;
check_obj! {
    #[derive(Debug,Deserialize)]
    pub struct MansionUncheck = MansionChecker > Mansion{
        pub id: IdChecker,
        #[serde(alias="cv_link")]
        pub link: MaxLimitString<128>,
        pub description:MaxLimitString<128>,
        pub fraction: FractionCheck,
        pub daily:IntoIterChecker<Vec<DailyUncheck>,DailyChecker,Vec<Daily>>
    }
    err:MansionDataError
}

#[derive(Debug)]
pub struct FractionCheck;

impl Checker for FractionCheck {
    type Args = ();
    type Checked = i16;
    type Err = MansionDataError;
    type Fut = Ready<Result<i16, MansionDataError>>;
    type Unchecked = i16;

    fn check(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        if (1..=5).contains(&uncheck) {
            ok(uncheck)
        }
        else {
            err(MansionDataError::BadFraction.into())
        }
    }
}
