use futures::Future;

use super::DataChecker;

/// 引用检查模式下的检查器
pub trait RefChecker {
    type Target;

    type Err;

    type Args;

    type Fut: Future<Output = Result<(), Self::Err>>;

    fn ref_checker(args: Self::Args, target: &Self::Target) -> Self::Fut;
}

impl<C> DataChecker for C
where
    C: RefChecker,
{
    type Args = C::Args;
    type Checked = C::Target;
    type Err = C::Err;
    type Unchecked = C::Target;

    type Fut = impl Future<Output = Result<Self::Checked, Self::Err>>;

    fn checker(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        async move {
            let fut = C::ref_checker(args, &uncheck);
            let _r = fut.await?;
            Ok(uncheck)
        }
    }
}
