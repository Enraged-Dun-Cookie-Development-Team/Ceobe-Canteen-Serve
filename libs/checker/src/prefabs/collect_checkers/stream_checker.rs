#![allow(dead_code)]

use std::{
    convert::Infallible,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{
    future::{ok, Ready},
    pin_mut, Future, Stream, StreamExt,
};

use crate::Checker;

#[pin_project::pin_project]
pub struct CheckedStream<S, C: Checker> {
    #[pin]
    stream: S,
    args: C::Args,
}

impl<S, C> Stream for CheckedStream<S, C>
where
    S: Stream,
    C: Checker<Unchecked = S::Item>,
    C::Args: Clone,
{
    type Item = Result<C::Checked, C::Err>;

    fn poll_next(
        self: Pin<&mut Self>, cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        let next_task = this.stream.next();
        let task = async move {
            match next_task.await {
                Some(uncheck) => {
                    let resp = C::check(this.args.clone(), uncheck).await;
                    Some(resp)
                }
                None => None,
            }
        };

        pin_mut!(task);
        task.poll(cx)
    }
}

pub struct StreamChecker<S, C>(PhantomData<S>, PhantomData<C>);

impl<S, C> Checker for StreamChecker<S, C>
where
    S: Stream,
    C: Checker<Unchecked = S::Item> + 'static,
    C::Args: Clone,
{
    type Args = C::Args;
    type Checked = CheckedStream<S, C>;
    type Err = Infallible;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = S;

    fn check(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ok(CheckedStream {
            stream: uncheck,
            args,
        })
    }
}
