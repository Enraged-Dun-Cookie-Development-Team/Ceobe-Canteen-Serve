#![allow(dead_code)]
use std::{marker::PhantomData, task::Poll};

use futures::{pin_mut, Future};

use crate::Checker;

/// 对 Slice 全部元素进行检查，一个错误就全部退出
#[derive(Debug)]
pub struct SliceChecker<S, C, O>(PhantomData<(S, C, O)>)
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: Checker<Unchecked = S::Item>,
    C::Args: Clone;

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

    fn checker(
        args: Self::Args, uncheck: Self::Unchecked,
    ) -> Self::Fut {
        SliceCheckerFut {
            args,
            iter: uncheck.into_iter(),
            result: Vec::new(),
            _phantom: PhantomData,
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
    _phantom: PhantomData<O>,
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

        let uncheck = match this.iter.next() {
            Some(uncheck) => uncheck,
            None => {
                let buf = std::mem::take(this.result);
                return Poll::Ready(Ok(O::from_iter(buf.into_iter())));
            }
        };
        let fut = C::checker(this.args.clone(), uncheck);

        pin_mut!(fut);

        match fut.poll(cx) {
            Poll::Ready(resp) => {
                match resp {
                    Ok(resp) => {
                        this.result.push(resp);
                        Poll::Pending
                    }
                    Err(err) => Poll::Ready(Err(err)),
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
