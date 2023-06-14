use std::{future::Future, marker::PhantomPinned, pin::Pin, task::{Context, Poll}};
use tokio::time::Interval;
struct TimeUsageExecute<Fut, Func> {
    future: Fut,
    execute: Func,
    interval:Interval,
    _phantom: PhantomPinned,
}

impl<Fut, Func> Future for TimeUsageExecute<Fut, Func>
where
    Fut: Future,
    Func: Fn(),
{
    type Output = Fut::Output;

    fn poll(
        self: Pin<&mut Self>, cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        todo!()
    }
}
