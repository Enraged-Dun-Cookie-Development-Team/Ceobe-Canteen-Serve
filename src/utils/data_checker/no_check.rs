use std::{convert::Infallible, marker::PhantomData};

use futures::future::ok;

use super::DataChecker;

pub struct NoCheck<T>(PhantomData<T>);

impl<T> DataChecker for NoCheck<T> {
    type Args = ();
    type Checked = T;
    type Err = Infallible;
    type Fut = futures_util::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = T;

    fn checker(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ok(uncheck)
    }
}
