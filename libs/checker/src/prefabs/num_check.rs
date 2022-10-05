use std::{fmt::Display, marker::PhantomData, ops::Deref};

use futures::future::{ok, ready, Ready};
use num_traits::{Unsigned, Zero};
use serde::Serialize;

use crate::Checker;

#[derive(Debug)]
pub struct NonZeroUnsignedError;
impl Display for NonZeroUnsignedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "预期非0值，取0")
    }
}
impl std::error::Error for NonZeroUnsignedError {}

pub struct NonZeroUnsignedChecker<T: Unsigned + Zero + 'static>(
    PhantomData<T>,
);

impl<T: Unsigned + Zero + 'static> Checker for NonZeroUnsignedChecker<T> {
    type Args = ();
    type Checked = NonZeroUnsigned<T>;
    type Err = NonZeroUnsignedError;
    type Fut = Ready<Result<NonZeroUnsigned<T>, Self::Err>>;
    type Unchecked = T;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        if uncheck.is_zero() {
            ready(Err(NonZeroUnsignedError))
        }
        else {
            ok(NonZeroUnsigned(uncheck))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NonZeroUnsigned<T: Unsigned + 'static>(T);

impl<T: Unsigned + 'static + Copy> Copy for NonZeroUnsigned<T> {}

impl<T: Unsigned + 'static> Deref for NonZeroUnsigned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}
