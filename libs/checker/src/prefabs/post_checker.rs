use std::marker::PhantomData;

use futures::{FutureExt, TryFutureExt};

use crate::{Checker, RefChecker};

pub struct PostChecker<C, P, E>(PhantomData<(C, P, E)>);

impl<C, P, E> Checker for PostChecker<C, P, E>
where
    C: Checker,
    P: RefChecker<Target = C::Checked>,
    E: From<C::Err>,
    E: From<P::Err>,
{
    type Unchecked = C::Unchecked;

    type Args = (C::Args, P::Args);

    type Checked = C::Checked;

    type Err = E;

    type Fut = BoxFuture<'static, Result<Self::Checked, Self::Err>>;

    fn check(
        (c_args, p_args): Self::Args, uncheck: Self::Unchecked,
    ) -> Self::Fut {
        Box::pin(async move {
            let checked = C::check(c_args, uncheck).await?;
            P::ref_checker(p_args, &checked).await?;
            Ok(checked)
        })
    }
}
