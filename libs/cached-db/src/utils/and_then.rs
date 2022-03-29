use std::future::Future;
#[derive(Debug)]
pub enum OptionError<E> {
    Inner(E),
    None,
}

pub struct AndThen<S>(pub(crate) S);

impl<S> AndThen<S> {
    pub fn new(inner: S) -> Self { Self(inner) }
}

impl<S, Req, T> tower::Service<Req> for AndThen<S>
where
    S: tower::Service<Req, Response = Option<T>>,
    T: Clone,
{
    type Error = OptionError<S::Error>;
    type Response = T;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self, cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx).map_err(OptionError::Inner)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let fut = self.0.call(req);

        async move {
            match fut.await.map_err(OptionError::Inner)? {
                Some(data) => Ok(data),
                None => Err(OptionError::None),
            }
        }
    }
}
