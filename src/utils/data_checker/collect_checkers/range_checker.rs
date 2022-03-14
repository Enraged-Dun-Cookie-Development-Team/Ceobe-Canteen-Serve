use futures_util::future::ready;

use futures::future::Ready;
use range_limit::RangeBoundLimit;

use crate::utils::data_checker::DataChecker;

impl<T, Rb> DataChecker for RangeBoundLimit<T, Rb>
where
    T: range_limit::measurable::Measurable,
    Rb: range_limit::RangeBound,
{
    type Unchecked = T;

    type Args = ();

    type Checked = T;

    type Err = range_limit::Error;

    type Fut = Ready<Result<T, Self::Err>>;

    fn checker(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let f = || {
            let v = Self::try_from(uncheck)?;
            Ok(v.into())
        };
        ready(f())
    }
}
