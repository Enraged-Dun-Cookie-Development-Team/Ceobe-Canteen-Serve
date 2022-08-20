use futures::future::{ready, Ready};
use range_limit::RangeBoundLimit;

use crate::Checker;

impl<T, Rb> Checker for RangeBoundLimit<T, Rb>
where
    T: range_limit::measurable::Measurable,
    Rb: range_limit::RangeBound,
{
    type Args = ();
    type Checked = T;
    type Err = range_limit::Error;
    type Fut = Ready<Result<T, Self::Err>>;
    type Unchecked = T;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let f = || {
            let v = Self::try_from(uncheck)?;
            Ok(v.into())
        };
        ready(f())
    }
}
