use std::{marker::PhantomData, fmt::Display};

use futures::future::{ready, Ready, ok};
use num_traits::{Unsigned, Zero};

use crate::Checker;

#[derive(Debug)]
pub struct NonZeroUnsignedError;
impl Display for NonZeroUnsignedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "预期非0值，取0"
        )
    }
}
impl std::error::Error for NonZeroUnsignedError {

}


pub struct NonZeroUnsignedChecker<T: Unsigned + Zero + 'static> (PhantomData<T>);

impl<T: Unsigned + Zero + 'static> Checker for NonZeroUnsignedChecker<T> {
    type Args = ();
    type Err = NonZeroUnsignedError;
    type Fut = Ready<Result<T, Self::Err>>;
    type Unchecked = T;
    type Checked = T;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        if uncheck.is_zero() {
            ready(Err(NonZeroUnsignedError))
        } else {
            ok(uncheck)
        }
    }
}

