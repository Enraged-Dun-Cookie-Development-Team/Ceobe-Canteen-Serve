use std::{convert::Infallible, marker::PhantomData};

use futures::future::ok;

use crate::AsyncChecker;

pub struct NoCheck<T: 'static>(PhantomData<T>);

impl<T> AsyncChecker for NoCheck<T> {
    type Args = ();
    type Checked = T;
    type Err = Infallible;
    type Fut = futures::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = T;

    fn async_checker(
        _args: Self::Args, uncheck: Self::Unchecked,
    ) -> Self::Fut {
        ok(uncheck)
    }
}
