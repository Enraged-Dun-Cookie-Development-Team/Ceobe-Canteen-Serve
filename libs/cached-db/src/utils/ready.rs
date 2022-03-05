use std::{future::Future, task::Poll};

pub struct Ready<I>(pub I);

impl<I> Future for Ready<I>
where
    I: Clone,
{
    type Output = I;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Poll::Ready(self.0.clone())
    }
}
