use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};

use super::{
    daily::{Daily, DailyChecker, DailyUncheck},
    id_checker::IdChecker,
};
use crate::{
    serves::mansion::error::{BadFraction, MansionError},
    utils::{
        data_checker::{
            collect_checkers::iter_checkers::IntoIterChecker, DataChecker,
        },
        data_struct::MaxLimitString,
    },
};

crate::check_obj! {
    {#[derive(Debug,Deserialize)]}
    {#[derive(Debug,Deserialize,Serialize)]}
    pub struct MansionUncheck = MansionChecker > Mansion{
        pub id: IdChecker,
        #[serde(alias="cvlink")]
        pub link: MaxLimitString<128>,
        pub description:MaxLimitString<128>,
        pub fraction: FractionCheck,
        pub daily:IntoIterChecker<Vec<DailyUncheck>,DailyChecker,Vec<Daily>>
    }
    err:MansionError
}

#[derive(Debug)]
pub struct FractionCheck;

impl DataChecker for FractionCheck {
    type Args = ();
    type Checked = i16;
    type Err = MansionError;
    type Fut = Ready<Result<i16, MansionError>>;
    type Unchecked = i16;

    fn checker(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        if 1 <= uncheck && uncheck <= 5 {
            ok(uncheck)
        }
        else {
            err(BadFraction.into())
        }
    }
}
