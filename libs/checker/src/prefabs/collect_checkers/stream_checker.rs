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

use crate::AsyncChecker;

#[pin_project::pin_project]
pub struct CheckedStream<S, C: AsyncChecker> {
    #[pin]
    stream: S,
    args: C::Args,
}

impl<S, C> Stream for CheckedStream<S, C>
where
    S: Stream,
    C: AsyncChecker<Unchecked = S::Item>,
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
                    let resp =
                        C::async_checker(this.args.clone(), uncheck).await;
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

impl<S, C> AsyncChecker for StreamChecker<S, C>
where
    S: Stream,
    C: AsyncChecker<Unchecked = S::Item> + 'static,
    C::Args: Clone,
{
    type Args = C::Args;
    type Checked = CheckedStream<S, C>;
    type Err = Infallible;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = S;

    fn async_checker(
        args: Self::Args, uncheck: Self::Unchecked,
    ) -> Self::Fut {
        ok(CheckedStream {
            stream: uncheck,
            args,
        })
    }
}
