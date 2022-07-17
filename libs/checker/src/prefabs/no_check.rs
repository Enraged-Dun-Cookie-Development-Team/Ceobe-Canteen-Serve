use std::{convert::Infallible, marker::PhantomData};

use futures::future::ok;

use crate::Checker;

pub struct NoCheck<T: 'static>(PhantomData<T>);

impl<T: 'static> NoCheck<T> {
    pub fn new() -> Self { Self(PhantomData) }
}

impl<T> Checker for NoCheck<T> {
    type Args = ();
    type Checked = T;
    type Err = Infallible;
    type Fut = futures::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = T;

    fn check(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ok(uncheck)
    }
}
