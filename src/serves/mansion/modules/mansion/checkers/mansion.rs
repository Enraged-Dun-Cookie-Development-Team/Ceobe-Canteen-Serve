use crate::utils::data_checker::collect_checkers::iter_checkers::IntoIterChecker;
use futures::future::{err, ok, ready, Ready};

use serde::{Deserialize, Serialize};

use crate::{
    serves::mansion::error::{BadFraction, MansionError, UnknownId},
    utils::{data_checker::DataChecker, data_struct::MaxLimitString},
};

use super::daily::{Daily, DailyChecker, DailyUncheck};

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

/// 饼学大厦号的检查器
/// ## Uncheck
/// [String](std::string::String)
/// ## Checked
/// (i32,i32)
#[derive(Debug)]
pub struct IdChecker;

impl DataChecker for IdChecker {
    type Unchecked = String;

    type Args = ();

    type Checked = (i32, i32);

    type Err = MansionError;

    type Fut = Ready<Result<Self::Checked, Self::Err>>;

    fn checker(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let task = move || {
            let mut sp = uncheck.split(".");
            let f = sp.next().ok_or(UnknownId)?;
            let main_id = f.trim().parse::<i32>().map_err(|_| UnknownId)?;
            let n = sp.next().unwrap_or("0");
            let sub_id = n.trim().parse::<i32>().map_err(|_| UnknownId)?;

            if let Some(_)=sp.next(){
                Err(UnknownId)?;
            }
            Ok((main_id, sub_id))
        };

        ready(task())
    }
}

#[derive(Debug)]
pub struct FractionCheck;

impl DataChecker for FractionCheck {
    type Unchecked = i16;

    type Args = ();

    type Checked = i16;

    type Err = MansionError;

    type Fut = Ready<Result<i16, MansionError>>;

    fn checker(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        if 1 <= uncheck && uncheck <= 5 {
            ok(uncheck)
        } else {
            err(BadFraction.into())
        }
    }
}
