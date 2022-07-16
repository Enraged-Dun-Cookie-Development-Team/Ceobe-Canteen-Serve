use std::{convert::Infallible, marker::PhantomData, task::Poll};

use futures::{
    future::{ok, Ready},
    pin_mut, Future, Stream,
};

use crate::AsyncChecker;

#[pin_project::pin_project]
pub struct LazyCheckedIter<I, C: AsyncChecker>(I, C::Args);

impl<I, C> Stream for LazyCheckedIter<I, C>
where
    I: Iterator,
    C: AsyncChecker<Unchecked = I::Item>,
    C::Args: Clone,
{
    type Item = Result<C::Checked, C::Err>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();

        match this.0.next() {
            Some(uncheck) => {
                let fut = C::checker(this.1.clone(), uncheck);
                pin_mut!(fut);
                let resp = fut.poll(cx);
                match resp {
                    Poll::Ready(r) => Poll::Ready(Some(r)),
                    Poll::Pending => Poll::Pending,
                }
            }
            None => Poll::Ready(None),
        }
    }
}

pub struct LazyIterChecker<I: 'static, C>(PhantomData<I>, PhantomData<C>);

impl<I, C> AsyncChecker for LazyIterChecker<I, C>
where
    I: Iterator + 'static,
    C: AsyncChecker<Unchecked = I::Item>,
    C::Args: Clone,
{
    type Args = C::Args;
    type Checked = LazyCheckedIter<I, C>;
    type Err = Infallible;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = I;

    fn checker(
        args: Self::Args, uncheck: Self::Unchecked,
    ) -> Self::Fut {
        ok(LazyCheckedIter(uncheck, args))
    }
}
