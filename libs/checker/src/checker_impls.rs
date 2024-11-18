#![allow(dead_code)]

use std::task::Poll;

use futures::{pin_mut, Future};

use crate::{
    checker::{Checker, LiteChecker, RefChecker},
    lite_args::LiteArgs,
    sync_check::SyncFuture,
};

impl<S> Checker for S
where
    S: RefChecker,
{
    type Args = S::Args;
    type Checked = S::Target;
    type Err = S::Err;
    type Fut = CheckRefFut<S>;
    type Unchecked = S::Target;

    fn check(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let ptr = Box::into_raw(Box::new(uncheck)) as *const S::Target;
        let ref_target = unsafe { ptr.as_ref() }.unwrap();
        let fut = S::ref_checker(args, ref_target);

        CheckRefFut { fut, data: ptr }
    }
}

#[pin_project::pin_project]
pub struct CheckRefFut<S: RefChecker> {
    #[pin]
    fut: S::Fut,
    data: *const S::Target,
}

impl<S> SyncFuture for CheckRefFut<S>
where
    S: RefChecker,
    S::Fut: SyncFuture,
{
    fn into_inner(self) -> Self::Output {
        self.fut.into_inner()?;
        let data = unsafe { Box::from_raw(self.data as *mut S::Target) };
        Ok(*data)
    }
}

unsafe impl<S: RefChecker> Send for CheckRefFut<S> {}

impl<S: RefChecker> Future for CheckRefFut<S> {
    type Output = Result<S::Target, S::Err>;

    fn poll(
        self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        let task = this.fut;

        pin_mut!(task);

        match task.poll(cx) {
            std::task::Poll::Ready(resp) => Poll::Ready(resp.map(|_i| {
                let data = *this.data;
                let data = unsafe { Box::from_raw(data as *mut S::Target) };
                *data
            })),
            std::task::Poll::Pending => Poll::Pending,
        }
    }
}

impl<C> LiteChecker for C
where
    C: Checker,
    <C as Checker>::Args: LiteArgs,
{
    fn lite_check(uncheck: Self::Unchecked) -> Self::Fut {
        <C as Checker>::check(
            <<C as Checker>::Args as LiteArgs>::get_arg(),
            uncheck,
        )
    }
}

#[cfg(test)]
mod test {
    use futures::future::{ready, Ready};

    use crate::{CheckRequire, RefChecker};

    struct CanSafeIntoU32Checker;
    #[derive(Debug, PartialEq, Eq)]
    struct OutOfRangeError;

    impl RefChecker for CanSafeIntoU32Checker {
        type Args = ();
        type Err = OutOfRangeError;
        type Fut = Ready<Result<(), Self::Err>>;
        type Target = i32;

        fn ref_checker(_: Self::Args, target: &Self::Target) -> Self::Fut {
            let res = if target >= &0 {
                Ok(())
            } else {
                Err(OutOfRangeError)
            };
            ready(res)
        }
    }

    #[tokio::test]
    async fn test_ref_checker() {
        let init_data = 12345i32;
        let resp = CanSafeIntoU32Checker::ref_checker((), &init_data).await;

        assert_eq!(Ok(()), resp)
    }
    #[tokio::test]
    async fn test_checker() {
        let init_data = CheckRequire::new(CanSafeIntoU32Checker, 114514i32);
        let resp = init_data.checking(()).await;

        assert_eq!(Ok(114514), resp);
    }
    #[tokio::test]
    async fn test_lite_checker() {
        let init = CheckRequire::new(CanSafeIntoU32Checker, 123456);
        let resp = init.lite_checking().await;

        assert_eq!(Ok(123456), resp)
    }
}
