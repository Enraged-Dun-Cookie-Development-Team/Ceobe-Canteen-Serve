use futures::future::{err, ok, ready, Ready};

use crate::{
    mansion::error::{BadFraction, MansionError, UnknownId},
    utils::{data_checker::DataChecker, data_struct::MaxLimitString},
};

crate::check_obj! {
    {}
    {}
    pub struct MansionUncheck = MansionChecker > Mansion{
        id : IdChecker,
        // link:MaxLimitString<128>
        fraction:FractionCheck
    }
    err:MansionError
}

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

pub struct FractionCheck;

impl DataChecker for FractionCheck {
    type Unchecked = i16;

    type Args = ();

    type Checked = i16;

    type Err = MansionError;

    type Fut = Ready<Result<i16, MansionError>>;

    fn checker(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        if 0 <= uncheck && uncheck <= 5 {
            ok(uncheck)
        } else {
            err(BadFraction.into())
        }
    }
}
