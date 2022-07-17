#![allow(dead_code)]

use std::task::Poll;

use futures::{pin_mut, Future};

use crate::checker::{Checker, RefChecker};

impl<S> Checker for S
where
    S: RefChecker,
{
    type Args = S::Args;
    type Checked = S::Target;
    type Err = S::Err;
    type Fut = CheckRef<S>;
    type Unchecked = S::Target;

    fn checker(
        args: Self::Args, uncheck: Self::Unchecked,
    ) -> Self::Fut {
        let ptr = Box::into_raw(Box::new(uncheck)) as *const S::Target;
        let ref_target = unsafe { ptr.as_ref() }.unwrap();
        let fut = S::ref_checker(args, ref_target);

        CheckRef { fut, data: ptr }
    }
}

#[pin_project::pin_project]
pub struct CheckRef<S: RefChecker> {
    #[pin]
    fut: S::Fut,
    data: *const S::Target,
}

impl<S: RefChecker> Future for CheckRef<S> {
    type Output = Result<S::Target, S::Err>;

    fn poll(
        self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        let task = this.fut;

        pin_mut!(task);

        match task.poll(cx) {
            std::task::Poll::Ready(resp) => {
                Poll::Ready(resp.map(|_i| {
                    let data = *this.data;
                    let data =
                        unsafe { Box::from_raw(data as *mut S::Target) };
                    *data
                }))
            }
            std::task::Poll::Pending => Poll::Pending,
        }
    }
}
