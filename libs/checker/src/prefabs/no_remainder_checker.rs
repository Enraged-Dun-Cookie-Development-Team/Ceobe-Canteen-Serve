use std::ops::Rem;

use futures::future::{ready, Ready};

use crate::RefChecker;

pub struct NoRemainderChecker<const RHS: u64>;

impl<const RHS: u64> RefChecker for NoRemainderChecker<RHS> {
    type Args = ();
    type Err = HasRemError<RHS>;
    type Fut = Ready<Result<(), Self::Err>>;
    type Target = u64;

    fn ref_checker(_: Self::Args, target: &Self::Target) -> Self::Fut {
        let rem = target.rem(RHS);
        ready(
            if rem == 0 {
                Ok(())
            }
            else {
                Err(HasRemError {
                    origin: *target,
                    rem,
                })
            },
        )
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("{origin} 无法被 {DIV} 整除，余数{rem}")]
pub struct HasRemError<const DIV: u64> {
    origin: u64,
    rem: u64,
}

#[cfg(test)]
mod test {
    use super::{HasRemError, NoRemainderChecker};
    use crate::RefChecker;

    #[test]
    fn test() {
        let a = 1111000;
        let _ = NoRemainderChecker::<1000>::ref_checker((), &a)
            .into_inner()
            .unwrap();

        let r = NoRemainderChecker::<10000>::ref_checker((), &a).into_inner();
        assert_eq!(
            Err(HasRemError::<10000> {
                origin: a,
                rem: a % 10000
            }),
            r
        )
    }
}
