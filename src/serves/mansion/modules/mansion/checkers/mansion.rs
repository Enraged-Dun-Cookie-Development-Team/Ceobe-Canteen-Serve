use futures::future::{err, ok, ready, Ready};
use sea_orm::Set;
use serde::Deserialize;

use crate::{utils::{data_struct::MaxLimitString, data_checker::DataChecker}, serves::mansion::{error::{MansionError, UnknownId, BadFraction}, db_ops}};


crate::check_obj! {
    {#[derive(Debug,Deserialize)]}
    {}
    pub struct MansionUncheck = MansionChecker > Mansion{
        id: IdChecker,
        link: MaxLimitString<128>,
        description:MaxLimitString<128>,
        fraction: FractionCheck
    }
    err:MansionError
}

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

impl Mansion {
    pub fn into_active_model_with_daily(self) -> db_ops::mansion::ActiveModel {
        let Mansion {
            id: (mid, sub_mid),
            link,
            description,
            fraction,
        } = self;

        db_ops::mansion::ActiveModel {
            mid: Set(mid),
            sub_mid: Set(sub_mid),
            description: Set(description),
            link: Set(link),
            fraction: Set(fraction),
            ..Default::default()
        }
    }
}
