use std::ops::Rem;

use futures::future::{ready, Ready};

use crate::RefChecker;

pub struct NoRemain<const RHS: u64>;

impl<const RHS: u64> RefChecker for NoRemain<RHS> {
    type Target = u64;

    type Err = HasRem<RHS>;

    type Args = ();

    type Fut = Ready<Result<(), Self::Err>>;

    fn ref_checker(_: Self::Args, target: &Self::Target) -> Self::Fut {
        let rem = target.rem(RHS);
        ready(if rem == 0 {
            Ok(())
        } else {
            Err(HasRem {
                origin: *target,
                rem,
            })
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("`{origin}` Has Rem for `{DIV}` with `{rem}`")]
pub struct HasRem<const DIV: u64> {
    origin: u64,
    rem: u64,
}
