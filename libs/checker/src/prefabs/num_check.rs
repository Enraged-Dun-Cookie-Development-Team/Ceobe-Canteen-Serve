use std::{marker::PhantomData, fmt::Display};

use futures::future::{ready, Ready, ok};
use num_traits::{Unsigned, Zero};

use crate::{RefChecker};

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

impl<T: Unsigned + Zero + 'static> RefChecker for NonZeroUnsignedChecker<T> {
    type Target = T;
    type Args = ();
    type Err = NonZeroUnsignedError;
    type Fut = Ready<Result<(), Self::Err>>;

    fn ref_checker(_: Self::Args, target: &Self::Target) -> Self::Fut {
        if target.is_zero() {
            ready(Err(NonZeroUnsignedError))
        } else {
            ok(())
        }
    }
}

