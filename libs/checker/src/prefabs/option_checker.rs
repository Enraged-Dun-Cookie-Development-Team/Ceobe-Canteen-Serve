use std::{marker::PhantomData, task::Poll};

use futures::Future;

use crate::AsyncChecker;

pub struct OptionChecker<C: AsyncChecker>(PhantomData<C>);

impl<C: AsyncChecker> AsyncChecker for OptionChecker<C> {
    type Args = C::Args;
    type Checked = Option<C::Checked>;
    type Err = C::Err;
    type Fut = OptionCheckerFut<C>;
    type Unchecked = Option<C::Unchecked>;

    fn checker(
        args: Self::Args, uncheck: Self::Unchecked,
    ) -> Self::Fut {
        match uncheck {
            Some(uncheck) => {
                OptionCheckerFut::Some(C::checker(args, uncheck))
            }
            None => OptionCheckerFut::None,
        }
    }
}

#[pin_project::pin_project(project=EnumProj)]
pub enum OptionCheckerFut<C: AsyncChecker> {
    None,
    Some(#[pin] <C as AsyncChecker>::Fut),
}

impl<C: AsyncChecker> Future for OptionCheckerFut<C> {
    type Output = Result<Option<C::Checked>, C::Err>;

    fn poll(
        self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        match self.project() {
            EnumProj::None => Poll::Ready(Ok(None)),
            EnumProj::Some(task) => {
                match task.poll(cx) {
                    Poll::Ready(result) => Poll::Ready(result.map(Some)),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}
