#![allow(dead_code)]

use std::{convert::Infallible, marker::PhantomData, pin::Pin, task::Poll};

use futures::{
    future::{ok, Ready},
    Future, Stream,
};

use crate::Checker;

#[pin_project::pin_project]
pub struct LazyCheckedStream<I, C: Checker>(I, C::Args, Option<C::Fut>);

impl<I, C> Stream for LazyCheckedStream<I, C>
where
    I: Iterator,
    C: Checker<Unchecked = I::Item>,
    C::Args: Clone,
{
    type Item = Result<C::Checked, C::Err>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if let Some(fut) = this.2 {
            let pin_fut = unsafe { Pin::new_unchecked(fut) };

            match pin_fut.poll(cx) {
                Poll::Ready(data) => {
                    this.2.take();
                    Poll::Ready(Some(data))
                }
                Poll::Pending => Poll::Pending,
            }
        }
        else {
            match this.0.next() {
                Some(uncheck) => {
                    let mut fut = C::check(this.1.clone(), uncheck);
                    let pin_fut = unsafe { Pin::new_unchecked(&mut fut) };
                    let resp = pin_fut.poll(cx);
                    match resp {
                        Poll::Ready(r) => Poll::Ready(Some(r)),
                        Poll::Pending => {
                            this.2.replace(fut);
                            Poll::Pending
                        }
                    }
                }
                None => Poll::Ready(None),
            }
        }
    }
}

pub struct LazyIterChecker<I, C>(PhantomData<(I, C)>)
where
    I: Iterator + 'static,
    C: Checker<Unchecked = I::Item>,
    C::Args: Clone;

impl<I, C> Checker for LazyIterChecker<I, C>
where
    I: Iterator + 'static,
    C: Checker<Unchecked = I::Item>,
    C::Args: Clone,
{
    type Args = C::Args;
    type Checked = LazyCheckedStream<I, C>;
    type Err = Infallible;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = I;

    fn check(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ok(LazyCheckedStream(uncheck, args, None))
    }
}
#[cfg(test)]
mod test {
    use std::{
        convert::Infallible, marker::PhantomData, pin::Pin, time::Duration,
    };

    use futures::{Future, StreamExt};

    use super::LazyIterChecker;
    use crate::{CheckRequire, RefChecker};

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
    async fn test_lazy() {
        let a = CheckRequire::new(
            LazyIterChecker::<_, DelayNoChecker>(PhantomData),
            vec![1i32, 2, 3, 4, 5, 6, 7].into_iter(),
        );

        let mut resp = a.lite_checking().await.unwrap();
        let mut expect_iter = vec![1i32, 2, 3, 4, 5, 6, 7].into_iter();
        while let (Some(expect), Some(data)) =
            (expect_iter.next(), resp.next().await)
        {
            assert_eq!(expect, data.unwrap())
        }
    }
}
