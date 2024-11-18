#![allow(dead_code)]

use std::{fmt::Debug, marker::PhantomData, pin::Pin, task::Poll};

use futures::Future;

use crate::{Checker, SyncFuture};

/// 对 Slice 全部元素进行检查，一个错误就全部退出
#[derive(Debug, Default)]
pub struct SliceChecker<S, C, O>
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: Checker<Unchecked = S::Item>,
    C::Args: Clone,
{
    _phantom: PhantomData<(S, C, O)>,
}

impl<S, C, O> SliceChecker<S, C, O>
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: Checker<Unchecked = S::Item>,
    C::Args: Clone,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<S, C, O> Checker for SliceChecker<S, C, O>
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: Checker<Unchecked = S::Item>,
    C::Args: Clone,
{
    type Args = C::Args;
    type Checked = O;
    type Err = C::Err;
    type Fut = SliceCheckerFut<S, O, C>;
    type Unchecked = S;

    fn check(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        SliceCheckerFut {
            args,
            iter: uncheck.into_iter(),
            result: Vec::new(),
            _phantom: PhantomData,
            pending: None,
        }
    }
}

#[pin_project::pin_project]
pub struct SliceCheckerFut<S, O, C>
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: Checker<Unchecked = S::Item>,
    C::Args: Clone,
{
    args: <C as Checker>::Args,
    iter: <S as IntoIterator>::IntoIter,
    result: Vec<C::Checked>,
    pending: Option<<C as Checker>::Fut>,
    _phantom: PhantomData<O>,
}

impl<S, O, C> SyncFuture for SliceCheckerFut<S, O, C>
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: Checker<Unchecked = S::Item>,
    C::Args: Clone,
    C::Fut : SyncFuture
{
    fn into_inner(mut self)->Self::Output {
        for uncheck in self.iter{
           let check_fut =  C::check(self.args.clone(), uncheck);
           let checked = SyncFuture::into_inner(check_fut)?;
           self.result.push(checked);
        }
        Ok(O::from_iter( self.result.into_iter()))
    }
}

impl<S, O, C> SliceCheckerFut<S, O, C>
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: Checker<Unchecked = S::Item>,
    C::Args: Clone,
{
}

impl<S, O, C> Future for SliceCheckerFut<S, O, C>
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: Checker<Unchecked = S::Item>,
    C::Args: Clone,
{
    type Output = Result<O, C::Err>;

    fn poll(
        self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.project();

        loop {
            // if has pending data
            // poll it
            if let Some(fut) = this.pending {
                let pin_fut = unsafe { Pin::new_unchecked(fut) };

                match pin_fut.poll(cx) {
                    Poll::Ready(Ok(resp)) => {
                        // ok,go ahead
                        this.result.push(resp);
                    }
                    // error occur return
                    Poll::Ready(Err(err)) => break Poll::Ready(Err(err)),
                    // still pending ,break and waiting next call
                    Poll::Pending => break Poll::Pending,
                }
            }

            // clear pending
            this.pending.take();
            // go ahead
            if let Some(uncheck) = this.iter.next() {
                let mut fut = C::check(this.args.clone(), uncheck);
                let pin_fut = unsafe { Pin::new_unchecked(&mut fut) };

                match pin_fut.poll(cx) {
                    Poll::Ready(Ok(resp)) => {
                        this.result.push(resp);
                        continue;
                    }
                    Poll::Ready(Err(err)) => break Poll::Ready(Err(err)),
                    Poll::Pending => {
                        this.pending.replace(fut);

                        break Poll::Pending;
                    }
                }
            }
            else {
                let buf = std::mem::take(this.result);
                break Poll::Ready(Ok(O::from_iter(buf)));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{convert::Infallible, pin::Pin, time::Duration};

    use futures::{pin_mut, Future};

    use super::SliceChecker;
    use crate::{prefabs::no_check::NoCheck, CheckRequire, RefChecker};

    #[tokio::test]
    async fn test_slice_checker() {
        let init = CheckRequire::new(
            SliceChecker::<_, NoCheck<_>, Vec<_>>::new(),
            vec![1i32, 23, 4, 4, 34, 44],
        );

        let resp = init.lite_checking();

        pin_mut!(resp);

        let task = tokio::time::timeout(Duration::from_secs(5), resp);

        let resp = task.await;

        assert_eq!(Ok(Ok(vec![1i32, 23, 4, 4, 34, 44])), resp)
    }

    struct DelayNoChecker;

    impl RefChecker for DelayNoChecker {
        type Args = ();
        type Err = Infallible;
        type Fut = Pin<Box<dyn Future<Output = Result<(), Self::Err>>>>;
        type Target = i32;

        fn ref_checker(_: Self::Args, _: &Self::Target) -> Self::Fut {
            Box::pin(async {
                tokio::time::sleep(Duration::from_secs(2)).await;
                tokio::time::sleep(Duration::from_secs(2)).await;
                Ok(())
            })
        }
    }

    #[tokio::test]
    async fn test_wait_checker() {
        let resp = CheckRequire::new(
            SliceChecker::<_, DelayNoChecker, Vec<_>>::new(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
        );

        let checking = resp.lite_checking().await;

        println!("{:?}", checking);
    }
}
