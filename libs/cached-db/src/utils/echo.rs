use std::{future::Future, task::Poll};

pub struct Echo;

impl<I> tower::Service<I> for Echo {
    type Response = I;

    type Error = ();

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: I) -> Self::Future {
        async move { Ok(req) }
    }
}
