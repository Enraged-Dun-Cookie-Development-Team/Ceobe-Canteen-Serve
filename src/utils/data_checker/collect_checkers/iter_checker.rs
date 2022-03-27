use futures_util::future::Ready;
use std::{convert::Infallible, marker::PhantomData, task::Poll};

use futures::{future::ok, pin_mut, Future, Stream};

use crate::utils::data_checker::DataChecker;

#[pin_project::pin_project]
pub struct CheckedIter<I, C: DataChecker>(I, C::Args);

impl<I, C> Stream for CheckedIter<I, C>
where
    I: Iterator,
    C: DataChecker<Unchecked = I::Item>,
    C::Args: Clone,
{
    type Item = Result<C::Checked, C::Err>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
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

pub struct IterChecker<I, C>(PhantomData<I>, PhantomData<C>);

impl<I, C> DataChecker for IterChecker<I, C>
where
    I: Iterator,
    C: DataChecker<Unchecked = I::Item>,
    C::Args: Clone,
{
    type Unchecked = I;

    type Args = C::Args;

    type Checked = CheckedIter<I, C>;

    type Err = Infallible;

    type Fut = Ready<Result<Self::Checked, Self::Err>>;

    fn checker(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ok(CheckedIter(uncheck, args))
    }
}
