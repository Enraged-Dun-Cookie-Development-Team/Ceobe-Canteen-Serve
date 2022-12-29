use std::ops::Rem;

use futures::future::{ready, Ready};

use crate::RefChecker;

pub struct NoRemainChecker<const RHS: u64>;

impl<const RHS: u64> RefChecker for NoRemainChecker<RHS> {
    type Args = ();
    type Err = HasRem<RHS>;
    type Fut = Ready<Result<(), Self::Err>>;
    type Target = u64;

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

#[derive(Debug, thiserror::Error, PartialEq)]
#[error("`{origin}` Has Rem for `{DIV}` with `{rem}`")]
pub struct HasRem<const DIV: u64> {
    origin: u64,
    rem: u64,
}

#[cfg(test)]
mod test {
    use crate::RefChecker;

    use super::{HasRem, NoRemainChecker};

    #[test]
    fn test() {
        let a = 1111000;
        let _ = NoRemainChecker::<1000>::ref_checker((), &a)
            .into_inner()
            .unwrap();

        let r = NoRemainChecker::<10000>::ref_checker((), &a).into_inner();
        assert_eq!(
            Err(HasRem::<10000> {
                origin: a,
                rem: a % 10000
            }),
            r
        )
    }
}
